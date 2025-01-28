#![no_std]
#![no_main]

use heapless::mpmc::MpMcQueue as Queue;
use log::{debug, error, info};

use libgreat::GreatResult;

use crate::hal::smolusb;
use smolusb::control::Control;
use smolusb::descriptor::{
    ConfigurationDescriptor, ConfigurationDescriptorHeader, DescriptorType, DeviceDescriptor,
    DeviceQualifierDescriptor, EndpointDescriptor, InterfaceDescriptor, InterfaceDescriptorHeader,
    LanguageId, StringDescriptor, StringDescriptorZero,
};
use smolusb::device::{Descriptors, Speed};
use smolusb::event::UsbEvent;
use smolusb::setup::SetupPacket;
use smolusb::traits::{ReadControl, ReadEndpoint, UnsafeUsbDriverOperations, UsbDriverOperations};

use moondancer::event::InterruptEvent;
use moondancer::{hal, pac};

// - constants ----------------------------------------------------------------

// TODO add support for other speeds
const DEVICE_SPEED: Speed = Speed::High;

// - global static state ------------------------------------------------------

static EVENT_QUEUE: Queue<InterruptEvent, 32> = Queue::new();

#[inline(always)]
fn dispatch_event(event: InterruptEvent) {
    match EVENT_QUEUE.enqueue(event) {
        Ok(()) => (),
        Err(_) => {
            error!("MachineExternal - event queue overflow");
            panic!("MachineExternal - event queue overflow");
        }
    }
}

// - MachineExternal interrupt handler ----------------------------------------

#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn MachineExternal() {
    use moondancer::UsbInterface::Target;

    let usb0 = unsafe { hal::Usb0::summon() };

    let pending = match pac::csr::interrupt::pending() {
        Ok(interrupt) => interrupt,
        Err(pending) => {
            dispatch_event(InterruptEvent::UnknownInterrupt(pending));
            return;
        }
    };

    match pending {
        // - usb0 interrupts - "target_phy" --

        // USB0 BusReset
        pac::Interrupt::USB0 => {
            usb0.device
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            usb0.bus_reset();
            dispatch_event(InterruptEvent::Usb(Target, UsbEvent::BusReset));
        }

        // USB0_EP_CONTROL ReceiveControl
        pac::Interrupt::USB0_EP_CONTROL => {
            usb0.ep_control
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            let endpoint = usb0.ep_control.status().read().epno().bits() as u8;
            let mut buffer = [0_u8; 8];
            let _bytes_read = usb0.read_control(&mut buffer);
            let setup_packet = SetupPacket::from(buffer);
            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::ReceiveSetupPacket(endpoint, setup_packet),
            ));
        }

        // USB0_EP_IN SendComplete
        pac::Interrupt::USB0_EP_IN => {
            usb0.ep_in
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            // TODO something a little safer would be nice
            let endpoint = usb0.ep_in.status().read().epno().bits() as u8;
            unsafe {
                usb0.clear_tx_ack_active(endpoint);
            }

            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::SendComplete(endpoint),
            ));
        }

        // USB0_EP_OUT ReceivePacket
        pac::Interrupt::USB0_EP_OUT => {
            usb0.ep_out
                .ev_pending()
                .modify(|r, w| w.mask().bit(r.mask().bit()));

            let endpoint = usb0.ep_out.status().read().epno().bits() as u8;

            // discard packets from Bulk OUT transfer endpoint
            /*if endpoint == 1 {
                //while usb0.ep_out.status().read().have().bit() {
                //    let _b = usb0.ep_out.data().read().byte().bits();
                //}
                usb0.ep_out.reset().write(|w| w.fifo().bit(true));
                usb0.ep_out_prime_receive(1);
                return;
            }*/

            dispatch_event(InterruptEvent::Usb(
                Target,
                UsbEvent::ReceivePacket(endpoint),
            ));
        }

        // - Unhandled Interrupt --
        _ => dispatch_event(InterruptEvent::UnhandledInterrupt(pending)),
    }
}

// - main entry point ---------------------------------------------------------

#[cfg(feature = "vexriscv")]
#[riscv_rt::pre_init]
unsafe fn pre_main() {
    pac::cpu::vexriscv::flush_icache();
    #[cfg(feature = "vexriscv_dcache")]
    pac::cpu::vexriscv::flush_dcache();
}

#[riscv_rt::entry]
fn main() -> ! {
    match main_loop() {
        Ok(()) => {
            error!("Firmware exited unexpectedly in main loop");
            panic!("Firmware exited unexpectedly in main loop")
        }
        Err(e) => {
            error!("Fatal error in firmware main loop: {}", e);
            panic!("Fatal error in firmware main loop: {}", e)
        }
    }
}

// - main loop ----------------------------------------------------------------

#[allow(clippy::too_many_lines)] // well that's just, like, your opinion man
fn main_loop() -> GreatResult<()> {
    let peripherals = pac::Peripherals::take().unwrap();
    let leds = &peripherals.LEDS;

    // initialize logging
    moondancer::log::init();
    info!("Logging initialized");

    // usb0: target
    let mut usb0 = hal::Usb0::new(
        peripherals.USB0,
        peripherals.USB0_EP_CONTROL,
        peripherals.USB0_EP_IN,
        peripherals.USB0_EP_OUT,
    );

    // usb0 control endpoint
    let mut control = Control::<_, { smolusb::EP_MAX_PACKET_SIZE }>::new(
        0,
        Descriptors {
            device_speed: DEVICE_SPEED,
            device_descriptor: USB_DEVICE_DESCRIPTOR,
            configuration_descriptor: USB_CONFIGURATION_DESCRIPTOR_0,
            other_speed_configuration_descriptor: Some(USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0),
            device_qualifier_descriptor: Some(USB_DEVICE_QUALIFIER_DESCRIPTOR),
            string_descriptor_zero: USB_STRING_DESCRIPTOR_0,
            string_descriptors: USB_STRING_DESCRIPTORS,
            microsoft10: None,
        }
        .set_total_lengths()
    );

    // connect device
    usb0.disconnect();
    unsafe { riscv::asm::delay(60_000_000); }
    usb0.connect(DEVICE_SPEED);
    info!("Connected USB0 device.");

    // enable interrupts
    unsafe {
        // set mstatus register: interrupt enable
        riscv::interrupt::enable();

        // set mie register: machine external interrupts enable
        riscv::register::mie::set_mext();

        // write csr: enable usb0 interrupts and events
        pac::csr::interrupt::enable(pac::Interrupt::USB0);
        pac::csr::interrupt::enable(pac::Interrupt::USB0_EP_CONTROL);
        pac::csr::interrupt::enable(pac::Interrupt::USB0_EP_IN);
        pac::csr::interrupt::enable(pac::Interrupt::USB0_EP_OUT);
        usb0.enable_events();
    }

    let mut test_command = TestCommand::Stop;
    let mut test_stats = TestStats::new();
    #[allow(clippy::cast_possible_truncation)]
    let test_data = {
        let mut test_data = [0_u8; smolusb::EP_MAX_PACKET_SIZE];
        for (n, value) in test_data
            .iter_mut()
            .enumerate()
            .take(smolusb::EP_MAX_PACKET_SIZE)
        {
            *value = (n % usize::from(u8::MAX)) as u8;
        }
        test_data
    };

    // prime the usb OUT endpoints we'll be using
    usb0.ep_out_prime_receive(0);
    usb0.ep_out_prime_receive(1);
    usb0.ep_out_prime_receive(2);

    let mut counter = 0;
    let mut rx_buffer: [u8; smolusb::EP_MAX_PACKET_SIZE] = [0; smolusb::EP_MAX_PACKET_SIZE];

    info!("Peripherals initialized, entering main loop.");

    loop {
        let mut queue_length = 0;

        while let Some(event) = EVENT_QUEUE.dequeue() {
            use moondancer::{event::InterruptEvent::*, UsbInterface::Target};
            use smolusb::event::UsbEvent::*;

            leds.output().write(|w| unsafe { w.bits(0) });

            match event {
                // - usb0 event handlers --

                // Usb0 received a control event
                Usb(
                    Target,
                    event @ (BusReset
                    | ReceiveControl(0)
                    | ReceiveSetupPacket(0, _)
                    | ReceivePacket(0)
                    | SendComplete(0)),
                ) => {
                    control.dispatch_event(&usb0, event);
                    if matches!(event, ReceivePacket(_)) {
                        // re-enable ep_out interface
                        usb0.ep_out_enable();
                    }
                }

                // Usb0 received packet
                Usb(Target, ReceivePacket(endpoint)) => {
                    let bytes_read = usb0.read(endpoint, &mut rx_buffer);
                    if bytes_read == 0 {
                        continue;
                    }

                    if endpoint == 1 {
                        leds.output().write(|w| unsafe { w.bits(0b11_1000) });
                        if counter % 100 == 0 {
                            log::trace!(
                                "{:?} .. {:?}",
                                &rx_buffer[0..8],
                                &rx_buffer[(bytes_read - 8)..]
                            );
                        }
                        counter += 1;

                    } else if endpoint == 2 {
                        info!("received command data from host: {} bytes", bytes_read);
                        let command = rx_buffer[0].into();
                        match (bytes_read, &command) {
                            (1, TestCommand::In) => {
                                info!("starting test: IN");
                                test_stats.reset();
                                test_command = TestCommand::In;
                            }
                            (1, TestCommand::Out) => {
                                info!("starting test: OUT");
                                test_stats.reset();
                                test_command = TestCommand::Out;
                            }
                            (1, TestCommand::Stop) => {
                                info!("stopping test: {:?}", command);
                                info!("  max write time: {}", test_stats.max_write_time);
                                info!("  min write time: {}", test_stats.min_write_time);
                                info!("  max flush time: {}", test_stats.max_flush_time);
                                info!("  min flush time: {}", test_stats.min_flush_time);
                                info!("  write count: {}", test_stats.write_count);
                                info!("  reset count: {}", test_stats.reset_count);
                                test_command = TestCommand::Stop;
                            }
                            (bytes_read, _) => {
                                error!(
                                    "received invalid command from host: {:?} (read {} bytes)",
                                    command, bytes_read,
                                );
                                error!(
                                    "{:?} .. {:?}",
                                    &rx_buffer[0..8],
                                    &rx_buffer[(bytes_read - 8)..bytes_read]
                                );
                            }
                        }

                    } else {
                        error!("received data on unknown endpoint: {}", endpoint);
                    }

                    // re-enable ep_out interface
                    usb0.ep_out_enable();
                }

                // Usb0 transfer complete
                Usb(Target, SendComplete(_endpoint)) => {
                    leds.output().write(|w| unsafe { w.bits(0b00_0111) });
                }

                // Error Message
                ErrorMessage(message) => {
                    error!("MachineExternal Error - {}", message);
                }

                // Unhandled event
                _ => {
                    error!("Unhandled event: {:?}", event);
                }
            }

            queue_length += 1;
        }

        // perform tests
        match test_command {
            TestCommand::In => test_in_speed(leds, &usb0, &test_data, &mut test_stats),
            TestCommand::Out => (),
            _ => (),
        }

        // queue diagnostics
        if queue_length > test_stats.max_queue_length {
            test_stats.max_queue_length = queue_length;
            debug!("max_queue_length: {}", test_stats.max_queue_length);
        }
    }
}

// - tests --------------------------------------------------------------------

/// Send test data to host as fast as possible
#[inline(always)]
fn test_in_speed(
    _leds: &pac::LEDS,
    usb0: &hal::Usb0,
    test_data: &[u8; smolusb::EP_MAX_PACKET_SIZE],
    test_stats: &mut TestStats,
) {
    // Passing in a fixed size slice ref is 4MB/s vs 3.7MB/s
    #[inline(always)]
    fn test_write_slice(
        usb0: &hal::Usb0,
        endpoint: u8,
        data: &[u8; smolusb::EP_MAX_PACKET_SIZE],
    ) -> bool {
        let mut did_reset = false;
        if usb0.ep_in.status().read().have().bit() {
            usb0.ep_in.reset().write(|w| w.fifo().bit(true));
            did_reset = true;
        }
        // 5.033856452242371MB/s.
        for byte in data {
            usb0.ep_in.data().write(|w| unsafe { w.byte().bits(*byte) });
        }
        // 6.392375785142406MB/s. - no memory access
        /*for n in 0..smolusb::EP_MAX_PACKET_SIZE {
            usb0.ep_in.data.write(|w| unsafe { w.data().bits((n % 256) as u8) });
        }*/
        usb0.ep_in
            .endpoint()
            .write(|w| unsafe { w.number().bits(endpoint & 0xf) });
        did_reset
    }

    // wait for fifo endpoint to be idle
    let ((), t_flush) = moondancer::profile!(
        let mut timeout = 100;
        while !usb0.ep_in.status().read().idle().bit() && timeout > 0 {
            timeout -= 1;
        }
    );

    // write data to endpoint fifo
    let (did_reset, t_write) = moondancer::profile!(
        //usb0.write(0x1, test_data.into_iter().copied()); false // 6780 / 5653 ~3.99MB/s
        //usb0.write_ref(0x1, test_data.iter()); false // 5663 / 5652 - ~4.02MB/s
        test_write_slice(usb0, 0x1, test_data) // 56533 / 5652 - ~4.04MB/s
    );
    test_stats.write_count += 1;

    // update stats
    test_stats.update_in(t_write, t_flush, did_reset);
}

// - types --------------------------------------------------------------------

#[derive(Debug, PartialEq)]
#[repr(u8)]
enum TestCommand {
    Stop,
    In = 0x23,
    Out = 0x42,
    Error = 0xff,
}

impl From<u8> for TestCommand {
    fn from(value: u8) -> Self {
        match value {
            0x23 => TestCommand::In,
            0x42 => TestCommand::Out,
            0xff => TestCommand::Error,
            _ => TestCommand::Stop,
        }
    }
}

struct TestStats {
    max_queue_length: usize,

    max_write_time: usize,
    min_write_time: usize,
    max_flush_time: usize,
    min_flush_time: usize,

    write_count: usize,
    reset_count: usize,
}

impl TestStats {
    const fn new() -> Self {
        Self {
            max_queue_length: 0,
            max_write_time: 0,
            min_write_time: usize::MAX,
            max_flush_time: 0,
            min_flush_time: usize::MAX,
            write_count: 0,
            reset_count: 0,
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    #[inline(always)]
    fn update_in(&mut self, t_write: usize, t_flush: usize, did_reset: bool) {
        if t_write > self.max_write_time {
            self.max_write_time = t_write;
        }
        if t_write < self.min_write_time {
            self.min_write_time = t_write;
        }
        if t_flush > self.max_flush_time {
            self.max_flush_time = t_flush;
        }
        if t_flush < self.min_flush_time {
            self.min_flush_time = t_flush;
        }
        if did_reset {
            self.reset_count += 1;
        }
    }
}

// - usb descriptors ----------------------------------------------------------

static USB_DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    bcdUSB: 0x0200,
    bDeviceClass: 0x00,
    bDeviceSubClass: 0x00,
    bDeviceProtocol: 0x00,
    bMaxPacketSize: 64,
    idVendor: cynthion::shared::usb::bVendorId::example,
    idProduct: cynthion::shared::usb::bProductId::example,
    iManufacturer: 1,
    iProduct: 2,
    iSerialNumber: 3,
    bNumConfigurations: 1,
    ..DeviceDescriptor::new()
};

static USB_DEVICE_QUALIFIER_DESCRIPTOR: DeviceQualifierDescriptor = DeviceQualifierDescriptor {
    bcdUSB: 0x0200,
    bDeviceClass: 0x00,
    bDeviceSubClass: 0x00,
    bDeviceProtocol: 0x00,
    bMaxPacketSize0: 64,
    bNumConfigurations: 1,
    bReserved: 0,
    ..DeviceQualifierDescriptor::new()
};

static USB_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor = ConfigurationDescriptor::new(
    ConfigurationDescriptorHeader {
        bConfigurationValue: 1,
        iConfiguration: 1,
        bmAttributes: 0x80, // 0b1000_0000 = bus-powered
        bMaxPower: 50,      // 50 * 2 mA = 100 mA
        ..ConfigurationDescriptorHeader::new()
    },
    &[InterfaceDescriptor::new(
        InterfaceDescriptorHeader {
            iInterfaceNumber: 0,
            bAlternateSetting: 0,
            bInterfaceClass: 0x00,
            bInterfaceSubClass: 0x00,
            bInterfaceProtocol: 0x00,
            iInterface: 2,
            ..InterfaceDescriptorHeader::new()
        },
        &[
            EndpointDescriptor {
                bEndpointAddress: 0x01, // OUT
                bmAttributes: 0x02,     // Bulk
                wMaxPacketSize: 512,
                bInterval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                bEndpointAddress: 0x02, // OUT - host commands
                bmAttributes: 0x02,     // Bulk
                wMaxPacketSize: 8,
                bInterval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                bEndpointAddress: 0x81, // IN
                bmAttributes: 0x02,     // Bulk
                wMaxPacketSize: 512,
                bInterval: 0,
                ..EndpointDescriptor::new()
            },
        ],
    )],
);

static USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor =
    ConfigurationDescriptor::new(
        ConfigurationDescriptorHeader {
            bDescriptorType: DescriptorType::OtherSpeedConfiguration as u8,
            bConfigurationValue: 1,
            iConfiguration: 1,
            bmAttributes: 0x80, // 0b1000_0000 = bus-powered
            bMaxPower: 50,      // 50 * 2 mA = 100 mA
            ..ConfigurationDescriptorHeader::new()
        },
        &[InterfaceDescriptor::new(
            InterfaceDescriptorHeader {
                iInterfaceNumber: 0,
                bAlternateSetting: 0,
                bInterfaceClass: 0x00,
                bInterfaceSubClass: 0x00,
                bInterfaceProtocol: 0x00,
                iInterface: 2,
                ..InterfaceDescriptorHeader::new()
            },
            &[
                EndpointDescriptor {
                    bEndpointAddress: 0x01, // OUT
                    bmAttributes: 0x02,     // Bulk
                    wMaxPacketSize: 64,
                    bInterval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    bEndpointAddress: 0x02, // OUT - host commands
                    bmAttributes: 0x02,     // Bulk
                    wMaxPacketSize: 8,
                    bInterval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    bEndpointAddress: 0x81, // IN
                    bmAttributes: 0x02,     // Bulk
                    wMaxPacketSize: 64,
                    bInterval: 0,
                    ..EndpointDescriptor::new()
                },
            ],
        )],
    );

static USB_STRING_DESCRIPTOR_0: StringDescriptorZero =
    StringDescriptorZero::new(&[LanguageId::EnglishUnitedStates]);
static USB_STRING_DESCRIPTOR_1: StringDescriptor =
    StringDescriptor::new(cynthion::shared::usb::bManufacturerString::bulk_speed_test);
static USB_STRING_DESCRIPTOR_2: StringDescriptor =
    StringDescriptor::new(cynthion::shared::usb::bProductString::bulk_speed_test);
static USB_STRING_DESCRIPTOR_3: StringDescriptor = StringDescriptor::new("0000000000000000");

static USB_STRING_DESCRIPTORS: &[&StringDescriptor] = &[
    &USB_STRING_DESCRIPTOR_1,
    &USB_STRING_DESCRIPTOR_2,
    &USB_STRING_DESCRIPTOR_3,
];
