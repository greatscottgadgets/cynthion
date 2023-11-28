///! `smolusb` device types

/// USB Speed
///
/// Note: These match UTMI xcvr_select constant so the mapping may not be correct for other contexts.
///       See: https://github.com/greatscottgadgets/luna/blob/main/luna/gateware/usb/usb2/__init__.py
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Speed {
    /// High speed (480 Mbps)
    High = 0,
    /// Full speed (12 Mbps)
    Full = 1,
    /// Low speed (1.5 Mbps)
    Low = 2,
    /// Super Speed (5/10 Gbps - includes SuperSpeed+)
    Super = 3,
    /// unsupported: compatibility fallback
    SuperPlus = 4,
    /// unsupported: compatibility fallback
    Unknown = 0xff,
}

impl From<u8> for Speed {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => Speed::High, // gateware gives 1
            1 => Speed::Full,
            2 => Speed::Low,
            3 => Speed::Super,
            _ => unimplemented!(),
        }
    }
}

impl Speed {
    /// Convert from a libusb speed constant to smolusb
    ///
    /// See: https://github.com/libusb/libusb/blob/6bf2db6feaf3b611c9adedb6c4962a07f5cb07ae/libusb/libusb.h#L1126
    pub fn from_libusb(value: u8) -> Self {
        match value {
            0 => Speed::Unknown,
            1 => Speed::Low,
            2 => Speed::Full,
            3 => Speed::High,
            4 => Speed::Super,
            5 => Speed::Super,
            _ => Speed::Unknown,
        }
    }

    pub fn to_libusb(&self) -> u8 {
        match self {
            Speed::Low => 1,
            Speed::Full => 2,
            Speed::High => 3,
            Speed::Super => 4,
            _ => 0,
        }
    }
}

/// TODO this is deprecated deleteme
mod deprecated {
    #![allow(dead_code, unused_imports, unused_variables)]

    use core::cell::RefCell;
    use core::sync::atomic::{AtomicU8, Ordering};

    use log::{debug, error, info, trace, warn};

    use crate::control_deprecated::{Control, ControlEvent};
    use crate::descriptor::*;
    use crate::error::{SmolError, SmolResult};
    use crate::event::UsbEvent;
    use crate::setup::{Direction, Feature, Recipient, Request, RequestType, SetupPacket};
    use crate::traits::AsByteSliceIterator;
    use crate::traits::UsbDriver;

    /// USB device state
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum DeviceState {
        None,
        Reset,
        Addressed,
        Configured,
        Suspended, // TODO first need to add suspend signal to eptri
    }

    /// A USB device
    ///
    /// `UsbDevice` implements the control portion of the USB
    /// specification and consists of:
    ///
    ///     * a hal driver
    ///     * a device descriptor
    ///     * a configuration descriptor
    ///     * a set of string descriptors
    ///
    pub struct UsbDevice<'a, D, const MAX_RECEIVE_SIZE: usize> {
        pub hal_driver: D,

        device_descriptor: DeviceDescriptor,
        configuration_descriptor: ConfigurationDescriptor<'a>,
        device_qualifier_descriptor: Option<DeviceQualifierDescriptor>,
        other_speed_configuration_descriptor: Option<ConfigurationDescriptor<'a>>,
        string_descriptor_zero: StringDescriptorZero<'a>,
        string_descriptors: &'a [&'a StringDescriptor<'a>],

        pub control: Control<'a, D, MAX_RECEIVE_SIZE>,

        pub state: RefCell<DeviceState>,
        pub current_configuration: AtomicU8,
        pub feature_remote_wakeup: bool,
        pub quirk_set_address_before_status: bool,

        pub cb_class_request: Option<
            fn(
                device: &UsbDevice<'a, D, MAX_RECEIVE_SIZE>,
                setup_packet: &SetupPacket,
                request: u8,
            ),
        >,
        pub cb_vendor_request: Option<
            fn(
                device: &UsbDevice<'a, D, MAX_RECEIVE_SIZE>,
                setup_packet: &SetupPacket,
                request: u8,
            ),
        >,
        pub cb_string_request: Option<
            fn(device: &UsbDevice<'a, D, MAX_RECEIVE_SIZE>, setup_packet: &SetupPacket, index: u8),
        >,
    }

    impl<'a, D, const MAX_RECEIVE_SIZE: usize> UsbDevice<'a, D, MAX_RECEIVE_SIZE>
    where
        D: UsbDriver,
    {
        pub fn new(
            hal_driver: D,
            device_descriptor: DeviceDescriptor,
            configuration_descriptor: ConfigurationDescriptor<'a>,
            string_descriptor_zero: StringDescriptorZero<'a>,
            string_descriptors: &'a [&'a StringDescriptor<'a>],
        ) -> Self {
            // calculate and update descriptor length fields
            // TODO this ain't great but it will do for now
            let mut configuration_descriptor = configuration_descriptor.clone();
            let _total_length = configuration_descriptor.set_total_length();

            Self {
                hal_driver,

                device_descriptor,
                configuration_descriptor,
                device_qualifier_descriptor: None,
                other_speed_configuration_descriptor: None,
                string_descriptor_zero,
                string_descriptors,

                control: Control::new(),

                state: DeviceState::None.into(),
                current_configuration: 0.into(),
                feature_remote_wakeup: false,
                quirk_set_address_before_status: false,

                cb_class_request: None,
                cb_vendor_request: None,
                cb_string_request: None,
            }
        }

        pub fn state(&self) -> DeviceState {
            *self.state.borrow()
        }

        pub fn set_device_qualifier_descriptor(
            &mut self,
            device_qualifier_descriptor: DeviceQualifierDescriptor,
        ) {
            self.device_qualifier_descriptor = Some(device_qualifier_descriptor);
        }

        pub fn set_other_speed_configuration_descriptor(
            &mut self,
            other_speed_configuration_descriptor: ConfigurationDescriptor<'a>,
        ) {
            // calculate and update descriptor length fields
            // TODO this ain't great but it will do for now
            let mut other_speed_configuration_descriptor =
                other_speed_configuration_descriptor.clone();
            other_speed_configuration_descriptor.set_total_length();
            self.other_speed_configuration_descriptor = Some(other_speed_configuration_descriptor);
        }
    }

    // Device connection
    impl<'a, D, const MAX_RECEIVE_SIZE: usize> UsbDevice<'a, D, MAX_RECEIVE_SIZE>
    where
        D: UsbDriver,
    {
        pub fn connect(&self, device_speed: super::Speed) {
            self.hal_driver.connect(device_speed);
        }

        pub fn disconnect(&self) {
            self.hal_driver.disconnect()
        }

        pub fn reset(&self) {
            self.hal_driver.reset();
            self.state.replace(DeviceState::Reset.into());
        }

        pub fn bus_reset(&self) {
            self.state.replace(DeviceState::Reset.into());
        }
    }

    // Control dispatch
    impl<'a, D, const MAX_RECEIVE_SIZE: usize> UsbDevice<'a, D, MAX_RECEIVE_SIZE>
    where
        D: UsbDriver,
    {
        /// Dispatches USB events for handling by Control
        ///
        /// Returns unhandled Control responses for further handling by the caller
        pub fn dispatch_control(
            &mut self,
            event: UsbEvent,
        ) -> SmolResult<Option<ControlEvent<'a, MAX_RECEIVE_SIZE>>> {
            match self.control.dispatch(&self.hal_driver, event)? {
                Some(
                    response @ ControlEvent {
                        endpoint_number,
                        setup_packet,
                        bytes_read,
                        ..
                    },
                ) => {
                    // probably a standard request that can be handled by UsbDevice
                    // TODO check direction and split setup_request into in/out
                    if bytes_read == 0 {
                        // try to handle the request but return packet to caller if we can't
                        match self.setup_request(endpoint_number, &setup_packet)? {
                            Some(_setup_packet) => Ok(Some(response)),
                            None => Ok(None),
                        }

                        // setup packet has a data stage, probably a class or vendor request
                    } else {
                        // TODO any scenario where control could be handling this unless we add support
                        //      for registering class/vendor handlers with UsbDevice?
                        Ok(Some(response))
                    }
                }
                None => Ok(None),
            }
        }
    }

    // SETUP request
    impl<'a, D, const MAX_RECEIVE_SIZE: usize> UsbDevice<'a, D, MAX_RECEIVE_SIZE>
    where
        D: UsbDriver,
    {
        pub fn setup_request(
            &mut self,
            _endpoint_number: u8,
            setup_packet: &SetupPacket,
        ) -> SmolResult<Option<SetupPacket>> {
            let request_type = setup_packet.request_type();
            let request = setup_packet.request();

            if matches!(request_type, RequestType::Standard) {
                debug!(
                    "SETUP {:?} {:?} {:?} {:?} 0x{:x} 0x{:x} {}",
                    setup_packet.recipient(),
                    setup_packet.direction(),
                    request_type,
                    request,
                    setup_packet.value,
                    setup_packet.index,
                    setup_packet.length
                );
            }

            match (&request_type, &request) {
                (RequestType::Standard, Request::SetAddress) => {
                    self.setup_set_address(setup_packet)?;
                }
                (RequestType::Standard, Request::GetDescriptor) => {
                    self.setup_get_descriptor(setup_packet)?;
                }
                (RequestType::Standard, Request::SetConfiguration) => {
                    self.setup_set_configuration(setup_packet)?;
                }
                (RequestType::Standard, Request::GetConfiguration) => {
                    self.setup_get_configuration(setup_packet)?;
                }
                (RequestType::Standard, Request::ClearFeature) => {
                    self.setup_clear_feature(setup_packet)?;
                }
                (RequestType::Standard, Request::SetFeature) => {
                    self.setup_set_feature(setup_packet)?;
                }
                (RequestType::Standard, Request::GetStatus) => {
                    self.setup_get_status(setup_packet)?;
                }
                (RequestType::Class, Request::ClassOrVendor(request)) => {
                    // if we have a callback handler, invoke it
                    if let Some(cb) = self.cb_class_request {
                        cb(self, setup_packet, *request);

                        // otherwise return the setup packet for the caller to handle
                    } else {
                        return Ok(Some(*setup_packet));
                    }
                }
                (RequestType::Vendor, Request::ClassOrVendor(request)) => {
                    // if we have a callback handler, invoke it
                    if let Some(cb) = self.cb_vendor_request {
                        cb(self, setup_packet, *request);
                    } else {
                        // otherwise return the setup packet for the caller to handle
                        return Ok(Some(*setup_packet));
                    }
                }
                _ => {
                    warn!("SETUP unhandled request {:?} {:?}", request_type, request);
                    return Ok(Some(*setup_packet));
                }
            }

            Ok(None)
        }

        // TODO move tx_ack_active flag logic to control.rs
        fn setup_set_address(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
            let address: u8 = (setup_packet.value & 0x7f) as u8;

            if self.quirk_set_address_before_status {
                warn!(
                    "UsbDevice::setup_set_address({}) quirk_set_address_before_status",
                    address
                );
                // activate new address
                self.hal_driver.set_address(address);
                self.state.replace(DeviceState::Addressed.into());

                // ack status
                self.hal_driver.ack(0, Direction::HostToDevice);
            } else {
                trace!("UsbDevice::setup_set_address({})", address);

                // set tx_ack_active flag
                // TODO a slighty safer approach would be nice
                unsafe {
                    self.hal_driver.set_tx_ack_active(0);
                }

                // respond with ack status first before changing device address
                self.hal_driver.ack(0, Direction::HostToDevice);

                // wait for the response packet to get sent
                // TODO a slightly safer approach would be nice
                loop {
                    let active = unsafe { self.hal_driver.is_tx_ack_active(0) };
                    if active == false {
                        break;
                    }
                }

                // activate new address
                self.hal_driver.set_address(address);
                self.state.replace(DeviceState::Addressed.into());
            }

            debug!(
                "SETUP setup_set_address() address:{} ({})",
                setup_packet.value, address
            );

            Ok(())
        }

        fn setup_get_descriptor(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
            // extract the descriptor type and number from our SETUP request
            let [descriptor_number, descriptor_type_bits] = setup_packet.value.to_le_bytes();
            let descriptor_type = match DescriptorType::try_from(descriptor_type_bits) {
                Ok(descriptor_type) => descriptor_type,
                Err(e) => {
                    warn!(
                        "SETUP stall: invalid descriptor type: {} {}",
                        descriptor_type_bits, descriptor_number
                    );
                    self.hal_driver.stall_control_request();
                    return Ok(());
                }
            };

            // if the host is requesting less than the maximum amount of data,
            // only respond with the amount requested
            let requested_length = setup_packet.length as usize;

            trace!(
                "  descriptor_type:{:?} descriptor_number:{} requested_length:{}",
                descriptor_type,
                descriptor_number,
                requested_length
            );

            match (&descriptor_type, descriptor_number) {
                (DescriptorType::Device, 0) => {
                    self.hal_driver
                        .write_ref(0, self.device_descriptor.as_iter().take(requested_length));
                }
                (DescriptorType::Configuration, 0) => {
                    self.hal_driver.write_ref(
                        0,
                        self.configuration_descriptor.iter().take(requested_length),
                    );
                }
                (DescriptorType::DeviceQualifier, 0) => {
                    if let Some(descriptor) = &self.device_qualifier_descriptor {
                        self.hal_driver
                            .write_ref(0, descriptor.as_iter().take(requested_length));
                    } else {
                        warn!("SETUP stall: no device qualifier descriptor configured");
                        // TODO stall?
                        return Ok(());
                    }
                }
                (DescriptorType::OtherSpeedConfiguration, 0) => {
                    if let Some(descriptor) = self.other_speed_configuration_descriptor {
                        self.hal_driver
                            .write_ref(0, descriptor.iter().take(requested_length));
                    } else {
                        warn!("SETUP stall: no other speed configuration descriptor configured");
                        // TODO stall?
                        return Ok(());
                    }
                }
                (DescriptorType::String, 0) => {
                    self.hal_driver
                        .write_ref(0, self.string_descriptor_zero.iter().take(requested_length));
                }
                (DescriptorType::String, index) => {
                    if let Some(cb) = self.cb_string_request {
                        cb(self, setup_packet, index);
                        return Ok(());
                    }

                    let offset_index: usize = (index - 1).into();
                    if offset_index > self.string_descriptors.len() {
                        warn!("SETUP stall: unknown string descriptor {}", index);
                        self.hal_driver.stall_control_request();
                        return Ok(());
                    }

                    self.hal_driver.write(
                        0,
                        self.string_descriptors[offset_index]
                            .iter()
                            .take(requested_length),
                    );
                }
                _ => {
                    warn!(
                        "SETUP stall: unhandled descriptor {:?}, {}",
                        descriptor_type, descriptor_number
                    );
                    self.hal_driver.stall_control_request();
                    return Ok(());
                }
            }

            self.hal_driver.ack(0, setup_packet.direction());

            Ok(())
        }

        fn setup_set_configuration(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
            self.hal_driver.ack(0, setup_packet.direction());

            let configuration: u8 = setup_packet.value as u8;

            trace!(
                "SETUP setup_set_configuration() configuration:{}",
                configuration
            );

            // TODO support multiple configurations
            if configuration > 1 {
                warn!("SETUP stall: unknown configuration {}", configuration);
                self.hal_driver.stall_control_request();
                return Ok(());
            }

            self.current_configuration
                .store(configuration, Ordering::Relaxed);
            self.state.replace(DeviceState::Configured.into());

            Ok(())
        }

        fn setup_get_configuration(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
            let requested_length = setup_packet.length as usize;

            trace!(
                "SETUP setup_get_configuration() requested_length:{}",
                requested_length
            );

            // handle unconfigured
            if self.state() != DeviceState::Configured {
                trace!("SETUP stall: setup_get_configuration() device is unconfigured");
                self.hal_driver.stall_control_request();
                return Ok(());
            }

            let current_configuration = self.current_configuration.load(Ordering::Relaxed);

            self.hal_driver.write_ref(0, [current_configuration].iter());
            self.hal_driver.ack(0, setup_packet.direction());

            Ok(())
        }

        fn setup_clear_feature(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
            // parse request
            let recipient = setup_packet.recipient();
            let feature_bits = setup_packet.value;
            let feature = match Feature::try_from(feature_bits) {
                Ok(feature) => feature,
                Err(e) => {
                    warn!("SETUP stall: invalid clear feature type: {}", feature_bits);
                    self.hal_driver.stall_control_request();
                    return Ok(());
                }
            };

            match (&recipient, &feature) {
                (Recipient::Device, Feature::DeviceRemoteWakeup) => {
                    // TODO self.feature_remote_wakeup = false;
                }
                (Recipient::Endpoint, Feature::EndpointHalt) => {
                    let endpoint_address = setup_packet.index as u8;
                    self.hal_driver
                        .clear_feature_endpoint_halt(endpoint_address);
                    self.hal_driver.ack(0, setup_packet.direction());
                    trace!(
                        "SETUP setup_clear_feature EndpointHalt: 0x{:x}",
                        endpoint_address
                    );
                }
                _ => {
                    warn!(
                        "SETUP stall: unhandled clear feature {:?}, {:?}",
                        recipient, feature
                    );
                    self.hal_driver.stall_control_request();
                    return Ok(());
                }
            };

            trace!("SETUP setup_clear_feature()");

            Ok(())
        }

        fn setup_set_feature(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
            trace!("SETUP setup_set_feature()");

            // parse request
            let recipient = setup_packet.recipient();
            let feature_bits = setup_packet.value;
            let feature = match Feature::try_from(feature_bits) {
                Ok(feature) => feature,
                Err(e) => {
                    warn!("SETUP stall: invalid set feature type: {}", feature_bits);
                    self.hal_driver.stall_control_request();
                    return Ok(());
                }
            };

            match (&recipient, &feature) {
                (Recipient::Device, Feature::DeviceRemoteWakeup) => {
                    // TODO self.feature_remote_wakeup = true;
                }
                _ => {
                    warn!(
                        "SETUP stall: unhandled set feature {:?}, {:?}",
                        recipient, feature
                    );
                    self.hal_driver.stall_control_request();
                    return Ok(());
                }
            };

            Ok(())
        }

        fn setup_get_status(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
            let recipient = setup_packet.recipient();

            log::info!("SETUP setup_get_status() recipient:{:?}", recipient);

            let status: u16 = 0b00; // TODO bit 1:remote-wakeup bit 0:self-powered

            self.hal_driver.write_ref(0, status.to_le_bytes().iter());
            self.hal_driver.ack(0, setup_packet.direction());

            Ok(())
        }
    }

    // Helpers
    //impl<'a, D, const MAX_RECEIVE_SIZE: usize> UsbDevice<'a, D, MAX_RECEIVE_SIZE> where D: UsbDriver {}

    /*
    # Reference enumeration process (quirks merged from Linux, macOS, and Windows):
    # - Read 8 bytes of device descriptor.
    # + Read 64 bytes of device descriptor.
    # + Set address.
    # + Read exact device descriptor length.
    # - Read device qualifier descriptor, three times.
    # - Read config descriptor (without subordinates).
    # - Read language descriptor.
    # - Read Windows extended descriptors. [optional]
    # - Read string descriptors from device descriptor (wIndex=language id).
    # - Set configuration.
    # - Read back configuration number and validate.

     */
}
