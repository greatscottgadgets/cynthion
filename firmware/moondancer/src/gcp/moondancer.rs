#![allow(dead_code, unused_imports, unused_variables)] // TODO

use crate::{hal, pac};
use pac::csr::interrupt;

use hal::smolusb;
use smolusb::control::{Direction, RequestType, SetupPacket};
use smolusb::device::{Speed, UsbDevice};
use smolusb::traits::{
    ControlRead, EndpointRead, EndpointWrite, EndpointWriteRef, UnsafeUsbDriverOperations,
    UsbDriverOperations,
};

use libgreat::error::{GreatError, GreatResult};
use libgreat::gcp::{self, Verb};

use log::{debug, error, trace, warn};
use zerocopy::{AsBytes, BigEndian, FromBytes, LittleEndian, Unaligned, U16, U32};

use core::any::Any;
use core::cell::RefCell;
use core::slice;

// - types --------------------------------------------------------------------

/// QuirkFlags
#[allow(non_snake_case, non_upper_case_globals)]
pub mod QuirkFlag {
    pub const SetAddressManually: u16 = 0x0001;
}

/// StatusRegister for status requests
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum StatusRegister {
    Interrupts = 0,
    QueuedSetupPackets = 1, // endpoints that have received a setup packet
    EndpointComplete = 2,   // endpoints that have completed a receive or transfer
}

impl TryFrom<u8> for StatusRegister {
    type Error = GreatError;
    fn try_from(value: u8) -> GreatResult<Self> {
        use StatusRegister::*;
        match value {
            0 => Ok(Interrupts),
            1 => Ok(QueuedSetupPackets),
            2 => Ok(EndpointComplete),
            _ => Err(GreatError::InvalidRequestCode),
        }
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod UsbInterruptFlag {
    pub const USB0_BUS_RESET: u32            = 1 << 0; // USB0
    pub const USB0_RECEIVE_SETUP_PACKET: u32 = 1 << 1; // USB0_EP_CONTROL
    pub const USB0_RECEIVE_CONTROL_DATA: u32 = 1 << 2; // USB0_EP_OUT (endpoint=0)
    pub const USB0_RECEIVE_DATA: u32         = 1 << 3; // USB0_EP_OUT
    pub const USB0_TRANSFER_COMPLETE: u32    = 1 << 4; // USB0_EP_IN
}

// - State --------------------------------------------------------------------

type ReceiveBuffer = [u8; crate::EP_MAX_PACKET_SIZE];

static mut SELF_STATE_RECEIVE_BUFFERS: [ReceiveBuffer; crate::EP_MAX_ENDPOINTS] = [[0; crate::EP_MAX_PACKET_SIZE]; crate::EP_MAX_ENDPOINTS];

/// State
struct State {
    /// an interrupt is pending
    usb0_interrupts_pending: u32, // 0x0 USBSTS

    /// bitmap: a setup packet awaits
    usb0_setup_pending: [Option<SetupPacket>; crate::EP_MAX_ENDPOINTS],

    /// bitmap: endpoints that have completed a transaction
    ///
    /// 00-15  receive complete
    /// 16-31  send complete
    usb0_endpoint_complete_pending: u32, // 0x2 ENDPTCOMPLETE


    //receive_buffers: [ReceiveBuffer; crate::EP_MAX_ENDPOINTS],
    bytes_read: [usize; crate::EP_MAX_ENDPOINTS],
}

impl Default for State {
    fn default() -> Self {
        Self {
            usb0_interrupts_pending: 0,
            usb0_setup_pending: [None; crate::EP_MAX_ENDPOINTS],
            usb0_endpoint_complete_pending: 0,
            //receive_buffers: [[0; crate::EP_MAX_PACKET_SIZE]; crate::EP_MAX_ENDPOINTS],
            bytes_read: [0; crate::EP_MAX_ENDPOINTS],
        }
    }
}

impl State {
    pub fn get(&mut self, register_type: &StatusRegister) -> u32 {
        let status: u32 = match register_type {
            StatusRegister::Interrupts => self.get_interrupts(),
            StatusRegister::QueuedSetupPackets => self.get_endpoint_setup_status(),
            StatusRegister::EndpointComplete => self.get_endpoint_complete(),
        };
        status
    }
}

impl State {
    // 0x0
    fn get_interrupts(&mut self) -> u32 {
        let status = self.usb0_interrupts_pending;
        self.usb0_interrupts_pending = 0;
        status
    }

    // 0x1
    fn get_endpoint_setup_status(&mut self) -> u32 {
        let bits = self.usb0_setup_pending.iter().enumerate().fold(0_u32, |bits, (endpoint_number, &bit)| {
            bits | (bit.is_some() as u32) << endpoint_number
        });

        debug!("MD Moondancer::State::get_endpoint_setup_status() -> 0b{:b}", bits);

        bits
    }

    // 0x2 - is there data waiting from a handle_receive or handle_transfer_complete?
    fn get_endpoint_complete(&mut self) -> u32 {
        let endpoint_complete_receive = self.usb0_endpoint_complete_pending;
        self.usb0_endpoint_complete_pending = 0;
        endpoint_complete_receive
    }
}

// - Moondancer --------------------------------------------------------------

/// Moondancer
pub struct Moondancer {
    pub usb0: hal::Usb0, // TODO needs to be private
    state: State,
    ep0_max_packet_size: u16,
    quirk_flags: u16,
}

impl Moondancer {
    pub fn new(usb0: hal::Usb0) -> Self {
        Self {
            usb0,
            state: State::default().into(),
            ep0_max_packet_size: 0,
            quirk_flags: 0,
        }
    }
}

// - usb0 interrupt handlers --------------------------------------------------

impl Moondancer {
    pub unsafe fn enable_usb_interrupts(&self) {
        interrupt::enable(pac::Interrupt::USB0);
        interrupt::enable(pac::Interrupt::USB0_EP_CONTROL);
        interrupt::enable(pac::Interrupt::USB0_EP_IN);
        interrupt::enable(pac::Interrupt::USB0_EP_OUT);

        // enable all usb events
        self.usb0.enable_interrupts();
    }

    pub unsafe fn disable_usb_interrupts(&self) {
        // disable all usb events
        self.usb0.disable_interrupts();

        interrupt::disable(pac::Interrupt::USB0);
        interrupt::disable(pac::Interrupt::USB0_EP_CONTROL);
        interrupt::disable(pac::Interrupt::USB0_EP_IN);
        interrupt::disable(pac::Interrupt::USB0_EP_OUT);
    }

    pub fn handle_bus_reset(&mut self) -> GreatResult<()> {
        self.state.usb0_interrupts_pending |= UsbInterruptFlag::USB0_BUS_RESET; // URI: USB reset received

        trace!(
            "MD => IRQ handle_bus_reset -> 0b{:b}",
            self.state.usb0_interrupts_pending
        );

        Ok(())
    }

    pub fn handle_receive_setup_packet(&mut self, endpoint_number: u8, setup_packet: SetupPacket) -> GreatResult<()> {
        trace!(
            "MD => IRQ handle_receive_setup_packet -> {} -> {:?} -> 0b{:b}",
            endpoint_number, setup_packet, self.state.usb0_interrupts_pending,
        );

        // TODO shouldn't clear_feature rather be handled in Facedancer?
        /*let request_type = setup_packet.request_type();
        let request = setup_packet.request();
        if request_type == RequestType::Standard && request == smolusb::control::Request::SetFeature {
            debug!("USB0 SETUP possible clear feature endpoint halt");
            let recipient = setup_packet.recipient();
            let feature_bits = setup_packet.value;
            let feature = match smolusb::control::Feature::try_from(feature_bits) {
                Ok(feature) => feature,
                Err(e) => {
                    warn!("SETUP stall: invalid clear feature type: {}", feature_bits);
                    self.usb0.stall_request();
                    return Ok(());
                }
            };
            if recipient == smolusb::control::Recipient::Endpoint && feature == smolusb::control::Feature::EndpointHalt {
                let endpoint_address = setup_packet.index as u8;
                self.usb0
                    .clear_feature_endpoint_halt(endpoint_address);
                self.usb0.ack_status_stage(&setup_packet);
                debug!(
                    "USB0 SETUP handle_clear_feature EndpointHalt: 0x{:x}",
                    endpoint_address
                );
            }
            return Ok(());
        }*/

        // TODO not sure yet whether this will be a problem in practice
        // TODO handle out of range
        if self.state.usb0_setup_pending[endpoint_number as usize].is_some() {
            warn!("MD queued setup packet for endpoint {} has not yet been transmitted", endpoint_number);
            //return Err(GreatError::DeviceOrResourceBusy);
        }

        self.state.usb0_interrupts_pending |= UsbInterruptFlag::USB0_RECEIVE_SETUP_PACKET;
        // TODO handle out of range
        self.state.usb0_setup_pending[endpoint_number as usize] = Some(setup_packet.clone());

        Ok(())
    }

    pub fn handle_receive_control_data(
        &mut self,
        bytes_read: usize,
        buffer: [u8; crate::EP_MAX_PACKET_SIZE],
    ) -> GreatResult<()> {
        self.state.usb0_interrupts_pending |= UsbInterruptFlag::USB0_RECEIVE_CONTROL_DATA;
        self.state.usb0_endpoint_complete_pending |= 1 << 0;

        trace!(
            "MD => IRQ handle_receive_control_data -> {} -> 0b{:b} -> 0b{:b}",
            bytes_read, self.state.usb0_interrupts_pending, self.state.usb0_endpoint_complete_pending,
        );

        self.state.bytes_read[0] = bytes_read;
        unsafe {
            //SELF_STATE_RECEIVE_BUFFERS[0] = buffer
            SELF_STATE_RECEIVE_BUFFERS[0].copy_from_slice(&buffer);
        };

        Ok(())
    }

    pub fn handle_receive_data(
        &mut self,
        endpoint: u8,
        bytes_read: usize,
        buffer: [u8; crate::EP_MAX_PACKET_SIZE],
    ) -> GreatResult<()> {
        self.state.usb0_interrupts_pending |= UsbInterruptFlag::USB0_RECEIVE_DATA;
        self.state.usb0_endpoint_complete_pending |= 1 << endpoint;

        trace!(
            "MD => IRQ handle_receive_data -> {} -> {} -> 0b{:b} -> 0b{:b}",
            endpoint,
            bytes_read,
            self.state.usb0_interrupts_pending,
            self.state.usb0_endpoint_complete_pending,
        );

        // TODO if endpoint > crate::EP_MAX_ENDPOINTS
        self.state.bytes_read[endpoint as usize] = bytes_read;
        unsafe {
            //SELF_STATE_RECEIVE_BUFFERS[endpoint as usize] = buffer
            SELF_STATE_RECEIVE_BUFFERS[endpoint as usize].copy_from_slice(&buffer);
        };

        Ok(())
    }

    pub fn handle_transfer_complete(&mut self, endpoint: u8) -> GreatResult<()> {
        self.state.usb0_interrupts_pending |= UsbInterruptFlag::USB0_TRANSFER_COMPLETE;
        self.state.usb0_endpoint_complete_pending |= 1 << (endpoint + 16);

        trace!(
            "MD => IRQ handle_transfer_complete -> {} -> 0b{:b}",
            endpoint, self.state.usb0_interrupts_pending
        );

        Ok(())
    }
}

// - verb implementations: connection / disconnection -------------------------

impl Moondancer {
    /// Connect the USB interface.
    pub fn connect(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            ep0_max_packet_size: U16<LittleEndian>,
            quirk_flags: U16<LittleEndian>,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        self.ep0_max_packet_size = args.ep0_max_packet_size.into();
        self.quirk_flags = args.quirk_flags.into();

        self.state = State::default();
        let speed = self.usb0.connect();

        unsafe { self.enable_usb_interrupts() };

        debug!(
            "MD Moondancer::connect(ep0_max_packet_size:{}, quirk_flags:{}) -> {:?}",
            args.ep0_max_packet_size, args.quirk_flags, speed
        );

        Ok([].into_iter())
    }

    /// Terminate all existing communication and disconnects the USB interface.
    pub fn disconnect(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        self.usb0.disconnect();
        self.state = State::default();

        debug!("MD Moondancer::disconnect()");

        Ok([].into_iter())
    }

    /// Perform a USB bus reset.
    pub fn bus_reset(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        self.state = State::default();
        self.usb0.bus_reset();

        debug!("MD Moondancer::bus_reset()");

        Ok([].into_iter())
    }
}

// - verb implementations: enumeration / setup --------------------------------

impl Moondancer {
    pub fn set_address(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            address: u8,
            deferred: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        // activate new address
        let address = args.address & 0x7f;
        self.usb0.set_address(address);

        debug!(
            "MD Moondancer::set_address(address:{}, deferred:{})",
            args.address, args.deferred
        );

        Ok([].into_iter())
    }

    pub fn set_up_endpoints(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(Debug, FromBytes, Unaligned)]
        struct ArgEndpoint {
            address: u8,
            max_packet_size: U16<LittleEndian>,
            transfer_type: u8,
        }

        // while we have endpoint triplets to handle
        let mut byte_slice = arguments;
        while let Some((endpoint, next)) =
            zerocopy::LayoutVerified::<_, ArgEndpoint>::new_from_prefix(byte_slice)
        {
            debug!("TODO MD Moondancer::set_up_endpoint(0x{:x}) -> 0x{:x} -> {}", endpoint.address, endpoint.address & 0x7f, endpoint.max_packet_size);
            byte_slice = next;

            // endpoint zero is always the control endpoint, and can't be configured
            if endpoint.address & 0x7f == 0x00 {
                warn!("MD ignoring request to reconfigure control endpoint");
                continue;
            }

            // prime any OUT endpoints
            if Direction::from_endpoint_address(endpoint.address) == Direction::HostToDevice {
                debug!("MD priming HostToDevice (OUT) endpoint address: {}", endpoint.address);
                self.usb0.ep_out_prime_receive(endpoint.address);
            }

            // ignore endpoint configurations we won't be able to handle
            if endpoint.max_packet_size.get() as usize > crate::EP_MAX_PACKET_SIZE {
                error!(
                    "MD failed to setup endpoint with max packet size {} > {}",
                    endpoint.max_packet_size,
                    crate::EP_MAX_PACKET_SIZE,
                );
                return Err(GreatError::InvalidArgument);
            }

            // TODO configure endpoint
        }

        let iter = [].into_iter();
        Ok(iter)
    }
}

// - verb implementations: status & control -----------------------------------

impl Moondancer {
    /// Query the Moondancer for any events that need to be processed.
    /// FIXME: should this actually use an interrupt pipe?
    ///
    /// The index value is used to select which status section we're looking for:
    ///
    ///	0 = pending interrupts (USBSTS register)
    ///	1 = setup status for all endpoints (ENDPTSETUPSTAT)
    ///	2 = endpoint completion status (ENDPTCOMPLETE)
    ///	3 = endpoint primed status (ENDPTSTATUS)
    ///
    ///	Returns: register_value: u32
    pub fn get_status(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            register_type: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let register_type = StatusRegister::try_from(args.register_type)?;
        let register_value = self.state.get(&register_type);

        // throttle log output some
        let mut is_repeat = false;
        unsafe {
            static mut LAST_TYPE: u8 = 0;
            static mut LAST_VALUE: u32 = 0;
            if args.register_type == LAST_TYPE && register_value == LAST_VALUE {
                is_repeat = true;
            }
            LAST_TYPE = args.register_type;
            LAST_VALUE = register_value;
        }

        if !is_repeat {
            trace!(
                "MD Moondancer::get_status(Args {{ register_type: {:?} }}) -> 0x{:x}",
                register_type, register_value
            );
        }

        let iter = register_value.to_le_bytes().into_iter();
        Ok(iter)
    }

    /// Read a setup packet from the Moondancer port and relays it to the host.
    ///
    /// The endpoint_number parameter specifies which endpoint we should be reading from.
    ///
    /// Always transmits an 8-byte setup packet back to the host. If no setup packet
    /// is waiting, the results of this vendor request are unspecified.
    ///
    /// Returns: raw_setup_packet: [u8; 8]
    pub fn read_setup(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            endpoint_number: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        // TODO handle out of range
        let result = match self.state.usb0_setup_pending[args.endpoint_number as usize].take() {
            Some(setup_packet) => Ok(SetupPacket::as_bytes(setup_packet).into_iter()),
            None => {
                error!("MD Moondancer::read_setup(endpoint_numger:{}) - no setup packet for endpoint",
                       args.endpoint_number);
                Err(GreatError::NoMessageOfType)
            },
        };

        trace!(
            "MD Moondancer::read_setup(endpoint_numger:{}) -> {:?}",
            args.endpoint_number, result
        );

        result
    }

    /// Temporarily stalls the given USB endpoint.
    pub fn stall_endpoint(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            endpoint_address: u8, // TODO consider using either endpoint_number or making _all_ api calls use address
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        self.usb0.stall_endpoint_address(args.endpoint_address, true);

        debug!("MD Moondancer::stall_endpoint({})", args.endpoint_address);

        Ok([].into_iter())
    }
}

// - verb implementations: data transfer --------------------------------------

impl Moondancer {
    /// Read data from the host and sends on the provided Moondancer endpoint.
    ///
    /// The OUT request should contain a data stage containing all data to be sent.
    pub fn send_on_endpoint(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        struct Args<B: zerocopy::ByteSlice> {
            endpoint_number: zerocopy::LayoutVerified<B, u8>,
            data_to_send: B,
        }
        let (endpoint_number, data_to_send) =
            zerocopy::LayoutVerified::new_unaligned_from_prefix(arguments)
                .ok_or(GreatError::InvalidArgument)?;
        let args = Args {
            endpoint_number,
            data_to_send,
        };

        let endpoint: u8 = args.endpoint_number.read();

        let iter = args.data_to_send.into_iter();
        self.usb0.write_ref(endpoint, iter);

        trace!(
            "MD Moondancer::send_on_endpoint(endpoint_number:{}, data_to_send.len:{})",
            endpoint,
            args.data_to_send.len()
        );

        Ok([].into_iter())
    }

    /// Prime the USB controller to recieve data on a particular endpoint.
    ///
    /// Does not wait for a transfer to complete. The transfer's
    /// status can be checked with `get_transfer_status` and then read
    /// with `finish_nonblocking_read`.
    ///
    /// TODO we might be able to drop this api call entirely
    pub fn start_nonblocking_read(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            endpoint_number: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        self.usb0.ep_out_prime_receive(args.endpoint_number);

        trace!(
            "MD Moondancer::start_nonblocking_read({})",
            args.endpoint_number
        );

        Ok([].into_iter())
    }

    /// Finish a non-blocking read by returning the read data back to the host.
    ///
    /// Returns: read_data: [u8]
    pub fn finish_nonblocking_read(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            endpoint_number: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        // TODO range check
        let endpoint_number = args.endpoint_number as usize;
        let data = unsafe { SELF_STATE_RECEIVE_BUFFERS[endpoint_number] };
        let bytes_read = self.state.bytes_read[endpoint_number];
        if bytes_read == 0 {
            error!("MD Moondancer::finish_nonblocking_read({}) no data available on endpoint", endpoint_number);
            // TODO error ?
        }
        self.state.bytes_read[endpoint_number] = 0;

        trace!(
            "MD Moondancer::finish_nonblocking_read({}) -> {}",
            endpoint_number, bytes_read,
        );

        let iter = data.into_iter().take(bytes_read);
        Ok(iter)
    }
}

// - class information --------------------------------------------------------

pub static CLASS: gcp::Class = gcp::Class {
    id: gcp::ClassId::moondancer,
    name: "moondancer",
    docs: CLASS_DOCS,
    verbs: &VERBS,
};

pub static CLASS_DOCS: &str = "API for fine-grained control of the Target USB port.\0";

/// Verb definitions for class: moondancer
///
/// Fields are `"\0"`  where C implementation has `""`
/// Fields are `"*\0"` where C implementation has `NULL`
pub static VERBS: [Verb; 13] = [
    // - connection / disconnection
    Verb {
        id: 0x0,
        name: "connect\0",
        doc: "\0", //"Setup the target port to connect to a host.\nEnables the target port's USB pull-ups.\0",
        in_signature: "<HH\0",
        in_param_names: "ep0_max_packet_size, quirk_flags\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x1,
        name: "disconnect\0",
        doc: "\0", //"Disconnect the target port from the host.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x2,
        name: "bus_reset\0",
        doc: "\0", //"Cause the target device to handle a bus reset.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },

    // - enumeration / setup --
    Verb {
        id: 0x3,
        name: "set_address\0",
        doc: "\0", //"Set the address of the target device.\nIf deferred is set this action won't complete until the setup phase ends.\0",
        in_signature: "<BB\0",
        in_param_names: "address, deferred\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x4,
        name: "set_up_endpoints\0",
        doc: "\0", //"Set up all of the non-control endpoints for the device.\0",
        in_signature: "<*(BHB)\0",
        in_param_names: "endpoint_descriptors\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },

    // - status & control --
    Verb {
        id: 0x5,
        name: "get_status\0",
        doc: "\0", //"Read one of the device's USB status registers.\0",
        in_signature: "<B\0",
        in_param_names: "register_type\0",
        out_signature: "<I\0",
        out_param_names: "register_value\0",
    },
    Verb {
        id: 0x6,
        name: "read_setup\0",
        doc: "\0", //"Read any pending setup packets recieved on the given endpoint.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_number\0",
        out_signature: "<8X\0",
        out_param_names: "raw_setup_packet\0",
    },
    Verb {
        id: 0x7,
        name: "stall_endpoint\0",
        doc: "\0", //"Stall the endpoint with the provided address.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_address\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },

    // - data transfer --
    Verb {
        id: 0x8,
        name: "send_on_endpoint\0",
        doc: "\0", //"Send the provided data on the given IN endpoint.\0",
        in_signature: "<B*X\0",
        in_param_names: "endpoint_number, data_to_send\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0xa,
        name: "start_nonblocking_read\0",
        doc: "\0", //"Begin listening for data on the given OUT endpoint.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_number\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0xb,
        name: "finish_nonblocking_read\0",
        doc: "\0", //"Return the data read after a given non-blocking read.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_number\0",
        out_signature: "<*X\0",
        out_param_names: "read_data\0",
    },

    Verb {
        id: 0x10,
        name: "get_event\0",
        doc: "\0", //"Return the most recent driver event. Returns TODO if there are none.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "<BB*X\0",
        out_param_names: "interface, type, data\0",
    },
    Verb {
        id: 0x11,
        name: "get_events\0",
        doc: "\0", //"Return the most recent driver events. Returns TODO if there are none.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "<*(BBB)\0",
        out_param_names: "interface, type, endpoint\0",
    },
];

// - verb implementations: new APIs -------------------------------------------

impl Moondancer {
    /// Get the most recent USB driver event.
    ///
    /// Returns: (type: u8, interface: u8, data: [u8])
    /// - interface: u8 -- interface number from 0 to 255
    /// - type: u8      -- where:
    ///     - None                  = 0x00
    ///     - UsbBusReset           = 0x10
    ///     - UsbReceiveSetupPacket = 0x11 (endpoint_number: u8, SetupPacket: [u8; 8])
    ///     - UsbReceivePacket      = 0x12 (endpoint_number: u8, bytes_read: u32, data: [u8])
    ///     - UsbSendComplete       = 0x13 (endpoint_number: u8)
    /// - data: [u8]    -- event data, parsed according to event type
    pub fn get_event(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        let response = [
            0x00,                                           // interface  = 0
            0x11,                                           // type       = UsbReceiveSetupPacket
            0x00,                                           // data[0]    = endpoint_number
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // data[1..8] = SetupPacket
        ];

        debug!("MD Moondancer::get_event() -> {:?}", response);

        Ok(response.into_iter())
    }

    /// Get the most recent USB driver events.
    ///
    /// Returns: [(interface, type, endpoint)]
    pub fn get_events(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {

        let response = [
            0x01_u8,                              // interface  = 0
            0x02_u8,                              // type       = UsbReceiveSetupPacket
            0x03_u8,                              // endpoint   = 0
        ].into_iter().chain([
        //    4_u32.to_le_bytes().into_iter()    // bytes_read = 8
        //).chain([
        //    0x0_u8, // pad
        //]).chain([
            0x5_u8,                               // interface  = 0
            0x6_u8,                               // type       = UsbReceivePacket
            0x7_u8,                               // endpoint   = 0
        //]).chain(
        //    8_u32.to_le_bytes().into_iter(), // bytes_read = 512
        //).chain([
        //    0x00_u8 // pad
        ]);

        debug!("MD Moondancer::get_events() -> {:?}", response);

        Ok(response)
    }

    pub fn read_endpoint(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        Ok([].into_iter())
    }

    pub fn write_endpoint(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        Ok([].into_iter())
    }
}


// - dispatch -----------------------------------------------------------------

use libgreat::gcp::{iter_to_response, GcpResponse, GCP_MAX_RESPONSE_LENGTH};

use core::{array, iter};

impl Moondancer {
    pub fn dispatch(
        &mut self,
        verb_number: u32,
        arguments: &[u8],
        response_buffer: [u8; GCP_MAX_RESPONSE_LENGTH],
    ) -> GreatResult<GcpResponse> {
        match verb_number {
            0x0 => {
                // moondancer::connect
                let iter = self.connect(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x1 => {
                // moondancer::disconnect
                let iter = self.disconnect(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x2 => {
                // moondancer::bus_reset
                let iter = self.bus_reset(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x3 => {
                // moondancer::set_address
                let iter = self.set_address(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x4 => {
                // moondancer::set_up_endpoints
                let iter = self.set_up_endpoints(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x5 => {
                // moondancer::get_status
                let iter = self.get_status(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x6 => {
                // moondancer::read_setup
                let iter = self.read_setup(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x7 => {
                // moondancer::stall_endpoint
                let iter = self.stall_endpoint(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x8 => {
                // moondancer::send_on_endpoint
                let iter = self.send_on_endpoint(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0xa => {
                // moondancer::start_nonblocking_read
                let iter = self.start_nonblocking_read(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0xb => {
                // moondancer::finish_nonblocking_read
                let iter = self.finish_nonblocking_read(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }

            // new APIs
            0x10 => {
                // moondancer::get_event
                let iter = self.get_event(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x11 => {
                // moondancer::get_events
                let iter = self.get_events(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }


            verb_number => Err(GreatError::InvalidArgument),
        }
    }
}
