//! Implementation for the GCP `gpio` class.

use zerocopy::byteorder::{LittleEndian, U16};
use zerocopy::{FromBytes, FromZeroes, Unaligned};

use crate::pac;

use libgreat::error::{GreatError, GreatResult};
use libgreat::gcp::{
    self, iter_to_response, GreatDispatch, GreatResponse, Verb, LIBGREAT_MAX_COMMAND_SIZE,
};

// - types --------------------------------------------------------------------

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod Port {
    pub const Gpio0: u8 = 0;
    pub const Gpio1: u8 = 1;
    pub const User0: u8 = 2;
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod _Mode {
    ///   0b00 - `Input-only` mode. The pin output is disabled but remains connected to its Output field.
    pub static InputOnly: u8 = 0b00;

    ///   0b01 - `Push-pull` mode. The pin output is enabled and connected to its Output field.
    pub static PushPull: u8 = 0b01;

    ///   0b10 - `Open-drain` mode. The pin output is enabled when the value of its Output field is 0
    ///          and is itself wired to 0.
    pub static OpenDrain: u8 = 0b10;

    ///   0b11 - `Alternate` mode. The pin output is disabled but remains connected to its Output field.
    pub static Alternate: u8 = 0b11;
}

#[repr(u8)]
pub enum Mode {
    InputOnly = 0b00,
    PushPull = 0b01,
    OpenDrain = 0b10,
    Alternate = 0b11,
}

// - Gpio ---------------------------------------------------------------------

/// Gpio
pub struct Gpio {
    gpio0: Option<pac::GPIO0>,
    gpio1: Option<pac::GPIO1>,
    user0: Option<pac::USER0>,
}

impl Gpio {
    #[must_use]
    pub fn new(
        gpio0: Option<pac::GPIO0>,
        gpio1: Option<pac::GPIO1>,
        user0: Option<pac::USER0>,
    ) -> Self {
        Self {
            gpio0,
            gpio1,
            user0,
        }
    }

    #[must_use]
    pub fn have_pin(&self, port: u8, pin: u8) -> bool {
        match (port, pin) {
            (Port::Gpio0, 0..=7) => self.gpio0.is_some(),
            (Port::Gpio1, 0..=7) => self.gpio1.is_some(),
            (Port::User0, 0) => self.user0.is_some(),
            _ => false,
        }
    }
}

// - verb implementations -----------------------------------------------------

impl Gpio {
    /// Configures a single pin to be used for GPIO.
    pub fn configure_pin(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            port: u8,
            pin: u8,
            initial_value: u8,
            mode: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        if !self.have_pin(args.port, args.pin) {
            log::error!(
                "gpio::configure_pin() - invalid port/pin: ({}, {}) ",
                args.port,
                args.pin
            );
            return Err(GreatError::InvalidArgument);
        }

        // set pin mode
        self.port_write_mode(args.port, args.pin, args.mode)?;

        // set initial value if it's an output pin
        if Self::is_output(args.mode) {
            self.port_write_output(args.port, args.pin, args.initial_value != 0)?;
        } else if args.initial_value != 0 {
            log::warn!(
                "gpion::configure_pin() - the given pin is not configured as an output: ({}, {})",
                args.port,
                args.pin
            );
        }

        Ok([].into_iter())
    }

    /// Releases a GPIO pin for use by other peripherals.
    pub fn release_pin(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            port: U16<LittleEndian>,
            pin: u8,
        }
        let _args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        // Not implemented at this time as our SoC's gpio setup is currently hard-wired. It would be
        // a fun exercise to be able to configure JTAG0 & UART1 duties for USER PMOD B at runtime.

        Ok([].into_iter())
    }

    /// Reads the pin mode register of a GPIO pin or pins given tuples of (port, pin).
    ///
    /// Returns the 2-bit [Mode] register block for each pin.
    pub fn get_pin_configurations(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct ArgPin {
            port: u8,
            number: u8,
        }

        // Cynthion has 2x 8 pin PMOD ports but we're only using USER PMOD A
        let mut configurations = [0_u8; 8];
        let mut count = 0;

        // while we have pins to handle
        let mut byte_slice = arguments;
        while let Some((pin, next)) = zerocopy::Ref::<_, ArgPin>::new_from_prefix(byte_slice) {
            if !self.have_pin(pin.port, pin.number) {
                log::error!(
                    "gpio::get_pin_configurations() - unknown port/pin: ({}, {}) ",
                    pin.port,
                    pin.number
                );
                return Err(GreatError::InvalidArgument);
            } else if count >= configurations.len() {
                log::warn!(
                    "gpio::get_pin_configurations() - can only return {} configurations.",
                    configurations.len()
                );
                return Err(GreatError::InvalidArgument);
            }

            let mode = self.port_read_mode(pin.port, pin.number)?;

            log::debug!(
                "#{} port:{} pin:{} mode:{:#04b}",
                count,
                pin.port,
                pin.number,
                mode
            );

            configurations[count] = mode;
            count += 1;
            byte_slice = next;
        }

        Ok(configurations.into_iter().take(count))
    }

    /// Reads the direction of a GPIO pin or pins given tuples of (port, pin).
    ///
    /// Returns 1 for output; 0 for input.
    pub fn get_pin_directions(
        &mut self,
        arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct ArgPin {
            port: u8,
            number: u8,
        }

        // Cynthion has 2x 8 pin PMOD ports but we're only using USER PMOD A
        let mut directions = [0_u8; 8];
        let mut count = 0;

        // while we have pins to handle
        let mut byte_slice = arguments;
        while let Some((pin, next)) = zerocopy::Ref::<_, ArgPin>::new_from_prefix(byte_slice) {
            if !self.have_pin(pin.port, pin.number) {
                log::error!(
                    "gpio::get_pin_directions() - unknown port/pin: ({}, {}) ",
                    pin.port,
                    pin.number
                );
                return Err(GreatError::InvalidArgument);
            } else if count >= directions.len() {
                log::warn!(
                    "gpio::get_pin_directions() - can only return {} configurations.",
                    directions.len()
                );
                return Err(GreatError::InvalidArgument);
            }

            let mode = self.port_read_mode(pin.port, pin.number)?;
            let direction =
                u8::from((mode == Mode::PushPull as u8) || (mode == Mode::OpenDrain as u8));

            log::debug!(
                "#{} port:{} pin:{} direction:{:#03b}",
                count,
                pin.port,
                pin.number,
                direction
            );

            directions[count] = direction;
            count += 1;
            byte_slice = next;
        }

        Ok(directions.into_iter().take(count))
    }

    /// Reads the value of a GPIO pin or pins given tuples of (port, pin).
    pub fn read_pins(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct ArgPin {
            port: u8,
            number: u8,
        }

        // Cynthion has 2x 8 pin PMOD ports but we're only using USER PMOD A
        let mut values = [0_u8; 8];
        let mut count = 0;

        // while we have pins to handle
        let mut byte_slice = arguments;
        while let Some((pin, next)) = zerocopy::Ref::<_, ArgPin>::new_from_prefix(byte_slice) {
            if !self.have_pin(pin.port, pin.number) {
                log::error!(
                    "gpio::read_pins() - unknown port/pin: ({}, {}) ",
                    pin.port,
                    pin.number
                );
                return Err(GreatError::InvalidArgument);
            } else if count >= values.len() {
                log::error!(
                    "gpio::read_pins() - can only return {} values.",
                    values.len()
                );
                return Err(GreatError::InvalidArgument);
            }

            let value = self.port_read_input(pin.port, pin.number)?;

            log::debug!(
                "#{} port:{} pin:{} value:{}",
                count,
                pin.port,
                pin.number,
                value
            );

            values[count] = u8::from(value);
            count += 1;
            byte_slice = next;
        }

        Ok(values.into_iter().take(count))
    }

    /// Sets the value of a GPIO pin or pins, given tuples of (port, pin, values).
    pub fn write_pins(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct ArgPin {
            port: u8,
            number: u8,
            value: u8,
        }

        // while we have pins to handle
        let mut byte_slice = arguments;
        while let Some((pin, next)) = zerocopy::Ref::<_, ArgPin>::new_from_prefix(byte_slice) {
            if !self.have_pin(pin.port, pin.number) {
                log::error!(
                    "gpio::write_pins() - unknown port/pin: ({}, {}) ",
                    pin.port,
                    pin.number
                );
                return Err(GreatError::InvalidArgument);
            }

            // get pin mode
            let mode = self.port_read_mode(pin.port, pin.number)?;

            // set value if it's an output pin
            if Self::is_output(mode) {
                self.port_write_output(pin.port, pin.number, pin.value != 0)?;
            } else {
                log::warn!(
                    "gpion::write_pins() - the given pin is not configured as an output: ({}, {})",
                    pin.port,
                    pin.number
                );
            }

            byte_slice = next;
        }

        Ok([].into_iter())
    }
}

// - helpers ------------------------------------------------------------------

impl Gpio {
    pub fn port_read_input(&self, port: u8, pin: u8) -> GreatResult<bool> {
        let value = match (port, pin) {
            (Port::Gpio0, 0..=7) => {
                let bits = self
                    .gpio0
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .input()
                    .read()
                    .bits();
                Self::read_input(bits, pin)
            }
            (Port::Gpio1, 0..=7) => {
                let bits = self
                    .gpio1
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .input()
                    .read()
                    .bits();
                Self::read_input(bits, pin)
            }
            (Port::User0, 0) => {
                let bits = self
                    .user0
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .input()
                    .read()
                    .bits();
                Self::read_input(bits, pin)
            }
            _ => return Err(GreatError::InvalidArgument),
        };
        Ok(value)
    }

    pub fn port_write_output(&self, port: u8, pin: u8, value: bool) -> GreatResult<()> {
        match (port, pin) {
            (Port::Gpio0, 0..=7) => {
                self.gpio0
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .output()
                    .modify(|r, w| {
                        let bits = Self::write_output(r.bits(), pin, value);
                        unsafe { w.bits(bits) }
                    });
            }
            (Port::Gpio1, 0..=7) => {
                self.gpio1
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .output()
                    .modify(|r, w| {
                        let bits = Self::write_output(r.bits(), pin, value);
                        unsafe { w.bits(bits) }
                    });
            }
            // Port::User0 is a button and cannot be written to.
            _ => return Err(GreatError::InvalidArgument),
        }
        Ok(())
    }

    pub fn port_write_mode(&self, port: u8, pin: u8, mode: u8) -> GreatResult<()> {
        match (port, pin, mode) {
            (Port::Gpio0, 0..=7, 0..=3) => {
                self.gpio0
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .mode()
                    .modify(|r, w| {
                        let bits = Self::write_mode(r.bits(), pin, mode);
                        unsafe { w.bits(bits) }
                    });
            }
            (Port::Gpio1, 0..=7, 0..=3) => {
                self.gpio1
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .mode()
                    .modify(|r, w| {
                        let bits = Self::write_mode(r.bits(), pin, mode);
                        unsafe { w.bits(bits) }
                    });
            }
            (Port::User0, 0, 0) => {
                self.user0
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .mode()
                    .modify(|r, w| {
                        let bits = Self::write_mode(r.bits().into(), pin, mode) as u8;
                        unsafe { w.bits(bits) }
                    });
            }
            _ => return Err(GreatError::InvalidArgument),
        }
        Ok(())
    }

    pub fn port_read_mode(&self, port: u8, pin: u8) -> GreatResult<u8> {
        let mode = match (port, pin) {
            (Port::Gpio0, 0..=7) => {
                let bits = self
                    .gpio0
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .mode()
                    .read()
                    .bits();
                Self::read_mode(bits, pin)
            }
            (Port::Gpio1, 0..=7) => {
                let bits = self
                    .gpio1
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .mode()
                    .read()
                    .bits();
                Self::read_mode(bits, pin)
            }
            (Port::User0, 0) => {
                let bits = self
                    .user0
                    .as_ref()
                    .ok_or(GreatError::InvalidArgument)?
                    .mode()
                    .read()
                    .bits();
                Self::read_mode(bits.into(), pin)
            }
            _ => return Err(GreatError::InvalidArgument),
        };
        Ok(mode)
    }

    /// Reads 1 bit at pin index.
    fn read_input(bits: u8, pin: u8) -> bool {
        assert!(u32::from(pin) < u8::BITS);

        bits & (0b1 << pin) != 0
    }

    /// Writes 1 bit at pin index.
    fn write_output(bits: u8, pin: u8, value: bool) -> u8 {
        assert!(u32::from(pin) < u8::BITS);
        let value = u8::from(value);

        let mask = (0b1 << pin) ^ (u8::pow(2, u8::BITS) - 1);
        (bits & mask) | (value << pin)
    }

    /// Reads 2 bits at pin index.
    fn read_mode(bits: u16, pin: u8) -> u8 {
        let index = pin * 2;
        assert!(u32::from(index) < u16::BITS);

        ((bits & (0b11 << index)) >> index) as u8
    }

    /// Writes 2 bits at pin index.
    fn write_mode(bits: u16, pin: u8, mode: u8) -> u16 {
        let index = pin * 2;
        let mode = u16::from(mode);
        assert!(u32::from(index) < u16::BITS);
        assert!(mode < 4);

        let mask = (0b11 << index) ^ (u16::pow(2, u16::BITS) - 1);
        (bits & mask) | (mode << index)
    }

    /// Returns true if the given mode is an output.
    fn is_output(mode: u8) -> bool {
        mode == Mode::PushPull as u8 || mode == Mode::OpenDrain as u8
    }
}

// - class information --------------------------------------------------------

pub static CLASS: gcp::Class = gcp::Class {
    id: gcp::ClassId::gpio,
    name: "gpio",
    docs: CLASS_DOCS,
    verbs: &VERBS,
};

pub static CLASS_DOCS: &str = "API for fine-grained control of the Target USB port.\0";

/// Verb definitions for class: gpio
///
/// Fields are `"\0"`  where C implementation has `""`
/// Fields are `"*\0"` where C implementation has `NULL`
pub static VERBS: [Verb; 6] = [
    Verb {
        id: 0x00,
        name: "configure_pin\0",
        doc: "\0", //"Configures a single pin to be used for GPIO.\0",
        in_signature: "<BBBB\0",
        in_param_names: "port, pin, initial_value, mode\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x01,
        name: "release_pin\0",
        doc: "\0", //"Releases a GPIO pin for use by other peripherals.\0",
        in_signature: "<BB\0",
        in_param_names: "port, pin\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x02,
        name: "get_pin_directions\0",
        doc: "\0", //"Reads the direction of a GPIO pin or pins given tuples of (port, pin). Returns 1 for output; 0 for input.\0",
        in_signature: "<*(BB)\0",
        in_param_names: "pins\0",
        out_signature: "<*B\0",
        out_param_names: "directions\0",
    },
    Verb {
        id: 0x03,
        name: "get_pin_configurations\0",
        doc: "\0", //"Reads the pin mode register of a GPIO pin or pins given tuples of (port, pin) Returns the 32 bit configuration register block for each pin.\0",
        in_signature: "<*(BB)\0",
        in_param_names: "pins\0",
        out_signature: "<*B\0",
        out_param_names: "configurations\0",
    },
    Verb {
        id: 0x04,
        name: "read_pins\0",
        doc: "\0", //"Reads the value of a GPIO pin or pins given tuples of (port, pin).\0",
        in_signature: "<*(BB)\0",
        in_param_names: "pins\0",
        out_signature: "<*B\0",
        out_param_names: "values\0",
    },
    Verb {
        id: 0x05,
        name: "write_pins\0",
        doc: "\0", //"Sets the value of a GPIO pin or pins, given tuples of (port, pin, values).\0",
        in_signature: "<*(BBB)\0",
        in_param_names: "pin_value_tuples\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
];

// - dispatch -----------------------------------------------------------------

#[allow(clippy::too_many_lines)]
impl GreatDispatch for Gpio {
    fn dispatch(
        &mut self,
        verb_number: u32,
        arguments: &[u8],
        response_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE],
    ) -> GreatResult<GreatResponse> {
        match verb_number {
            0x00 => {
                // gpio::set_up_pin
                let iter = self.configure_pin(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x01 => {
                // gpio::release_pin
                let iter = self.release_pin(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x02 => {
                // gpio::get_pin_directions
                let iter = self.get_pin_directions(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x03 => {
                // gpio::get_pin_configurations
                let iter = self.get_pin_configurations(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x04 => {
                // gpio::read_pins
                let iter = self.read_pins(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x05 => {
                // gpio::write_pins
                let iter = self.write_pins(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            _verb_number => Err(GreatError::InvalidArgument),
        }
    }
}
