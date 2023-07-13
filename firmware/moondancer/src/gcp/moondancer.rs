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
        id: 0x9,
        name: "clean_up_transfer\0",
        doc: "\0", //"Clean up any complete transfers on the given endpoint.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_address\0",
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
        id: 0xc,
        name: "get_nonblocking_data_length\0",
        doc: "\0", //"Return the amount of data read after a given non-blocking read.\0",
        in_signature: "<B\0",
        in_param_names: "endpoint_number\0",
        out_signature: "<I\0",
        out_param_names: "length\0",
    },
];

// - types --------------------------------------------------------------------

/// QuirkFlags
#[allow(non_snake_case, non_upper_case_globals)]
pub mod QuirkFlag {
    pub const SetAddressManually: u16 = 0x0001;
}

/// RegisterType for status requests
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum RegisterType {
    UsbStatus = 0,
    EndpointSetupStatus = 1,
    EndpointComplete = 2,
    EndpointStatus = 3,
    EndpointNak = 4,
}

impl TryFrom<u8> for RegisterType {
    type Error = GreatError;
    fn try_from(value: u8) -> GreatResult<Self> {
        use RegisterType::*;
        match value {
            0 => Ok(UsbStatus),
            1 => Ok(EndpointSetupStatus),
            2 => Ok(EndpointComplete),
            3 => Ok(EndpointStatus),
            4 => Ok(EndpointNak),
            _ => Err(GreatError::InvalidRequestCode),
        }
    }
}

// - State --------------------------------------------------------------------

type ReceiveBuffer = [u8; crate::EP_MAX_PACKET_SIZE];

/// State
struct State {
    /// an interrupt is pending
    usb0_status_pending: u32, // 0x0 USBSTS

    /// bitmap: a setup packet awaits
    ///
    /// TODO Option<(endpoint, SetupPacket)>
    /// TODO multiple queued packets?
    usb0_setup_pending: Option<SetupPacket>, // 0x1 ENDPTSETUPSTATE

    /// bitmap: endpoints that have completed a transaction
    ///
    /// 00-15  receive complete
    /// 16-31  send complete
    usb0_endpoint_complete_pending: u32, // 0x2 ENDPTCOMPLETE

    usb0_endpoint_prime_pending: u32, // 0x3 ENDPTSTATUS
    usb0_endpoint_nak_pending: u32,   // 0x4 ENDPTNAK

    receive_buffers: [ReceiveBuffer; crate::EP_MAX_ENDPOINTS],
    bytes_read: [usize; crate::EP_MAX_ENDPOINTS],
}

impl Default for State {
    fn default() -> Self {
        Self {
            usb0_status_pending: 0,
            usb0_setup_pending: None,
            usb0_endpoint_complete_pending: 0,
            usb0_endpoint_prime_pending: 0,
            usb0_endpoint_nak_pending: 0,
            receive_buffers: [[0; crate::EP_MAX_PACKET_SIZE]; crate::EP_MAX_ENDPOINTS],
            bytes_read: [0; crate::EP_MAX_ENDPOINTS],
        }
    }
}

impl State {
    pub fn get(&mut self, register_type: &RegisterType) -> u32 {
        let status: u32 = match register_type {
            RegisterType::UsbStatus => self.get_usb_status(),
            RegisterType::EndpointSetupStatus => self.get_endpoint_setup_status(),
            RegisterType::EndpointComplete => self.get_endpoint_complete(),
            RegisterType::EndpointStatus => self.get_endpoint_status(),
            RegisterType::EndpointNak => self.get_endpoint_nak(),
        };
        status
    }
}

impl State {
    // 0x0
    fn get_usb_status(&mut self) -> u32 {
        let status = self.usb0_status_pending;
        self.usb0_status_pending = 0;
        status
    }

    // 0x1
    fn get_endpoint_setup_status(&mut self) -> u32 {
        match self.usb0_setup_pending.is_some() {
            true => 1,
            false => 0,
        }
    }

    // 0x2 - is there data waiting from a handle_receive ?
    fn get_endpoint_complete(&mut self) -> u32 {
        let endpoint_complete_receive = self.usb0_endpoint_complete_pending;
        self.usb0_endpoint_complete_pending = 0;
        endpoint_complete_receive
    }

    // 0x3
    // aka usb_endpoint_is_ready - which queries Moondancer for a
    // bitmap describing the endpoints that are not currently primed,
    // and thus ready to be primed again
    fn get_endpoint_status(&mut self) -> u32 {
        let endpoint_prime = self.usb0_endpoint_prime_pending;
        self.usb0_endpoint_prime_pending = 0;
        endpoint_prime
    }

    // 0x4
    fn get_endpoint_nak(&mut self) -> u32 {
        let endpoint_nak = self.usb0_endpoint_nak_pending;
        self.usb0_endpoint_nak_pending = 0;
        endpoint_nak
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod UsbStatusFlag {
    /// UI: USB interrupt
    pub const USBSTS_D_UI: u32 = 1 << 0;

    pub const USBSTS_D_RECEIVE_SETUP_PACKET: u32 = 1 << 1;
    pub const USBSTS_D_RECEIVE_CONTROL_DATA: u32 = 1 << 2;
    pub const USBSTS_D_RECEIVE_DATA: u32 = 1 << 3;
    pub const USBSTS_D_SEND_COMPLETE: u32 = 1 << 4;

    /*
    /// UEI: USB error interrupt
    pub const USBSTS_D_UEI: u32 = 1 << 1;
    /// PCI: Port change detect
    pub const USBSTS_D_PCI: u32 = 1 << 2;
     */
    /// URI: USB reset received
    pub const USBSTS_D_URI: u32 = 1 << 6;
    /*
    /// SRRI: SOF received
    pub const USBSTS_D_SRI: u32 = 1 << 7;
    /// SLI: DCSuspend
    pub const USBSTS_D_SLI: u32 = 1 << 8;
    */
    /// NAKI: NAK interrupt bit
    pub const USBSTS_D_NAKI: u32 = 1 << 16;
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
        self.state.usb0_status_pending |= UsbStatusFlag::USBSTS_D_URI; // URI: USB reset received

        trace!(
            "MD => IRQ handle_bus_reset -> 0b{:b}",
            self.state.usb0_status_pending
        );

        Ok(())
    }

    pub fn handle_receive_setup_packet(&mut self, setup_packet: SetupPacket) -> GreatResult<()> {
        trace!(
            "MD => IRQ handle_receive_setup_packet -> {:?} -> 0b{:b}",
            setup_packet, self.state.usb0_status_pending,
        );

        // TODO not sure yet whether this will be a problem in practice
        if self.state.usb0_setup_pending.is_some() {
            error!("MD =>     queued setup packet has not yet been transmitted");
            //return Err(GreatError::DeviceOrResourceBusy);
        }

        self.state.usb0_status_pending |= UsbStatusFlag::USBSTS_D_RECEIVE_SETUP_PACKET;
        self.state.usb0_setup_pending = Some(setup_packet.clone());

        Ok(())
    }

    pub fn handle_receive_control_data(
        &mut self,
        bytes_read: usize,
        buffer: [u8; crate::EP_MAX_PACKET_SIZE],
    ) -> GreatResult<()> {
        self.state.usb0_status_pending |= UsbStatusFlag::USBSTS_D_RECEIVE_CONTROL_DATA;
        self.state.usb0_endpoint_complete_pending |= 1 << 0;

        trace!(
            "MD => IRQ handle_receive_control_data -> {} -> 0b{:b} -> 0b{:b}",
            bytes_read, self.state.usb0_status_pending, self.state.usb0_endpoint_nak_pending,
        );

        self.state.bytes_read[0] = bytes_read;
        self.state.receive_buffers[0] = buffer;

        Ok(())
    }

    pub fn handle_receive_data(
        &mut self,
        endpoint: u8,
        bytes_read: usize,
        buffer: [u8; crate::EP_MAX_PACKET_SIZE],
    ) -> GreatResult<()> {
        self.state.usb0_status_pending |= UsbStatusFlag::USBSTS_D_RECEIVE_DATA;
        self.state.usb0_endpoint_complete_pending |= 1 << endpoint;

        debug!(
            "MD => IRQ handle_receive_data -> {} -> {} -> 0b{:b} -> 0b{:b}",
            endpoint,
            bytes_read,
            self.state.usb0_status_pending,
            self.state.usb0_endpoint_nak_pending,
        );

        // TODO if endpoint > crate::EP_MAX_ENDPOINTS
        self.state.bytes_read[endpoint as usize] = bytes_read;
        self.state.receive_buffers[endpoint as usize] = buffer;

        Ok(())
    }

    pub fn handle_transfer_complete(&mut self, endpoint: u8) -> GreatResult<()> {
        self.state.usb0_status_pending |= UsbStatusFlag::USBSTS_D_SEND_COMPLETE;
        self.state.usb0_endpoint_complete_pending |= 1 << (endpoint + 16);

        trace!(
            "MD => IRQ handle_transfer_complete -> {} -> 0b{:b}",
            endpoint, self.state.usb0_status_pending
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

        debug!(
            "MD Moondancer::connect(ep0_max_packet_size:{}, quirk_flags:{}) -> {:?}",
            args.ep0_max_packet_size, args.quirk_flags, speed
        );

        unsafe { self.enable_usb_interrupts() };

        Ok([].into_iter())
    }

    /// Terminate all existing communication and disconnects the USB interface.
    pub fn disconnect(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        debug!("MD Moondancer::disconnect()");

        self.state = State::default();

        self.usb0.disconnect();

        let iter = [].into_iter();
        Ok(iter)
    }

    /// Perform a USB bus reset.
    pub fn bus_reset(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        trace!("MD Moondancer::bus_reset()");

        self.state = State::default();
        self.usb0.bus_reset();

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
        trace!(
            "MD Moondancer::set_address(address:{}, deferred:{})",
            args.address, args.deferred
        );

        // activate new address
        let address = args.address & 0x7f;
        self.usb0.set_address(address);

        let iter = [].into_iter();
        Ok(iter)
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
            trace!("TODO MD Moondancer::set_up_endpoint({:?})", endpoint);
            byte_slice = next;

            // endpoint zero is always the control endpoint, and can't be configured
            if endpoint.address & 0x7f == 0x00 {
                warn!("MD ignoring request to reconfigure control endpoint");
                continue;
            }

            // ignore endpoint configurations we won't be able to handle
            if endpoint.max_packet_size.get() as usize > self.state.receive_buffers[0].len() {
                error!(
                    "MD failed to setup endpoint with max packet size {} > {}",
                    endpoint.max_packet_size,
                    self.state.receive_buffers[0].len(),
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
        let register_type = RegisterType::try_from(args.register_type)?;
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

        // TODO handle endpoint numbers other than 0
        let result = match self.state.usb0_setup_pending.take() {
            Some(setup_packet) => Ok(SetupPacket::as_bytes(setup_packet).into_iter()),
            None => Err(GreatError::NoMessageOfType),
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
            endpoint_number: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        self.usb0.stall_endpoint_address(args.endpoint_number, true);

        trace!("MD Moondancer::stall_endpoint({})", args.endpoint_number);

        let iter = [].into_iter();
        Ok(iter)
    }
}

// - verb implementations: data transfer --------------------------------------

impl Moondancer {
    /// Read data from the GreatFET host and sends on the provided Moondancer endpoint.
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

    /// Should be called whenever a transfer is complete; cleans up any transfer
    /// descriptors associated with that transfer.
    pub fn clean_up_transfer(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            endpoint_address: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let endpoint_number = args.endpoint_address & 0x7f;
        trace!(
            "MD Moondancer::clean_up_transfer({} / 0x{:x})",
            args.endpoint_address, endpoint_number
        );

        let iter = [].into_iter();
        Ok(iter)
    }

    /// Prime the USB controller to recieve data on a particular endpoint.
    ///
    /// Does not wait for a transfer to complete. The transfer's
    /// status can be checked with `get_transfer_status` and then read
    /// with `finish_nonblocking_read`.
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

        debug!(
            "MD Moondancer::start_nonblocking_read({})",
            args.endpoint_number
        );

        let iter = [].into_iter();
        Ok(iter)
    }

    /// Finish a non-blocking read by returning the read data back to the host.
    ///
    /// This should only be used after determining that a transfer is
    /// complete with the `get_transfer_status` request and reading
    /// the relevant length with `get_nonblocking_data_length`.
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

        let endpoint = args.endpoint_number as usize;
        let data = self.state.receive_buffers[endpoint];
        let bytes_read = self.state.bytes_read[endpoint];
        self.state.bytes_read[endpoint] = 0;

        debug!(
            "MD Moondancer::finish_nonblocking_read({}) -> {}",
            endpoint, bytes_read,
        );

        let iter = data.into_iter().take(bytes_read);
        Ok(iter)
    }

    /// Query an endpoint to determine how much data is available.
    ///
    /// This should only be used after a nonblocking read was primed
    /// with `start_nonblocking_read` and completed by the USB
    /// hardware.
    ///
    /// Response is invalid unless a transfer has been initiated with
    /// `start_nonblocking_read` and completed.
    ///
    /// Returns: length: u32
    pub fn get_nonblocking_data_length(
        &self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            endpoint_number: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        debug!(
            "MD Moondancer::get_nonblocking_data_length({})",
            args.endpoint_number
        );
        let iter = [].into_iter();
        Ok(iter)
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
            0x9 => {
                // moondancer::clean_up_transfer
                let iter = self.clean_up_transfer(arguments)?;
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
            0xc => {
                // moondancer::get_nonblocking_data_length
                let iter = self.get_nonblocking_data_length(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }

            verb_number => Err(GreatError::InvalidArgument),
        }
    }
}
