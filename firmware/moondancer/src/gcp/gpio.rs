use log::{debug, error};
use zerocopy::{AsBytes, BigEndian, FromBytes, LittleEndian, Unaligned, U32};

/// Verbs for class: Gpio
#[repr(u32)]
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Gpio {
    /// Configures a single pin to be used for GPIO.
    /// .out_signature = "", .in_param_names = "port, pin, as_output, initial_value",
    set_up_pin = 0x0,
    /// Releases a GPIO pin for use by other peripherals.
    /// .out_signature = "", .in_param_names = "port, pin"
    release_pin = 0x1,
    /// Reads the direction of a GPIO pin or pins given tuples of (port, pin).
    /// Returns 1 for output; 0 for input.
    /// .out_signature = "<*B", .in_param_names = "pins", .out_param_names = "directions"
    get_pin_directions = 0x2,
    /// Reads the value of a GPIO pin or pins given tuples of (port, pin).
    /// .out_signature = "<*B", .in_param_names = "pins", .out_param_names = "values"
    read_pins = 0x3,
    /// Sets the value of a GPIO pin or pins, given tuples of (port, pin, values).
    /// .out_signature = "", .in_param_names = "pin_value_tuples"
    write_pins = 0x4,

    /// Unsupported verb
    unsupported(u32),
}

impl core::convert::From<u32> for Gpio {
    fn from(verb: u32) -> Self {
        match verb {
            0x0 => Gpio::set_up_pin,
            _ => Gpio::unsupported(verb),
        }
    }
}

impl core::convert::From<U32<LittleEndian>> for Gpio {
    fn from(value: U32<LittleEndian>) -> Self {
        Gpio::from(value.get())
    }
}

/// Dispatch
pub struct Dispatch {}

impl Dispatch {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Dispatch {
    pub fn handle(&self, class: super::Class, verb: Gpio) -> &[u8] {
        match verb {
            Gpio::set_up_pin => set_up_pin(),
            _ => {
                error!("unknown verb: {:?}.{:?}", class, verb);
                &[]
            }
        }
    }
}

// - verb implementations -----------------------------------------------------

pub fn set_up_pin() -> &'static [u8] {
    &[]
}
