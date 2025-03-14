//! Implementation for the GCP `led` class.

use zerocopy::{FromBytes, FromZeroes, Unaligned};

use crate::pac;

use libgreat::error::{GreatError, GreatResult};
use libgreat::gcp::{
    self, iter_to_response, GreatDispatch, GreatResponse, Verb, LIBGREAT_MAX_COMMAND_SIZE,
};


// - Leds ---------------------------------------------------------------------

/// Led
pub struct Leds {
    leds: pac::LEDS,
}

impl Leds {
    #[must_use]
    pub fn new(leds: pac::LEDS) -> Self {
        // leds: initialize all leds to off
        leds.output().write(|w| unsafe { w.bits(0b00_0000) });

        Self { leds }
    }
}

// - verb implementations -----------------------------------------------------

impl Leds {
    /// Toggle a LED.
    pub fn toggle(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            led_num: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        if args.led_num >= 6 {
            log::error!("leds::toggle() - invalid led number: ({}) ",
                        args.led_num);
            return Err(GreatError::InvalidArgument);
        }

        self.leds.output().modify(|r, w| {
            let bits = u8::pow(2, args.led_num.into()) ^ r.bits();
            unsafe { w.bits(bits) }
        });

        Ok([].into_iter())
    }

    /// Turn a LED on.
    pub fn on(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            led_num: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        if args.led_num >= 6 {
            log::error!("leds::on() - invalid led number: ({}) ",
                        args.led_num);
            return Err(GreatError::InvalidArgument);
        }

        self.leds.output().modify(|r, w| {
            let bits = u8::pow(2, args.led_num.into()) | r.bits();
            unsafe { w.bits(bits) }
        });

        Ok([].into_iter())
    }

    /// Turn a LED off.
    pub fn off(&mut self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, FromZeroes, Unaligned)]
        struct Args {
            led_num: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

        if args.led_num >= 6 {
            log::error!("leds::off() - invalid led number: ({}) ",
                        args.led_num);
            return Err(GreatError::InvalidArgument);
        }

        self.leds.output().modify(|r, w| {
            let bits = !u8::pow(2, args.led_num.into()) & r.bits();
            unsafe { w.bits(bits) }
        });

        Ok([].into_iter())
    }
}

// - class information --------------------------------------------------------

pub static CLASS: gcp::Class = gcp::Class {
    id: gcp::ClassId::leds,
    name: "leds",
    docs: CLASS_DOCS,
    verbs: &VERBS,
};

pub static CLASS_DOCS: &str = "API for LED configuration.\0";

/// Verb definitions for class: leds
///
/// Fields are `"\0"`  where C implementation has `""`
/// Fields are `"*\0"` where C implementation has `NULL`
pub static VERBS: [Verb; 3] = [
    Verb {
        id: 0x00,
        name: "toggle\0",
        doc: "\0", //"Toggle a LED.\0",
        in_signature: "<B\0",
        in_param_names: "led_num\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x01,
        name: "on\0",
        doc: "\0", //"Turn a LED on.\0",
        in_signature: "<B\0",
        in_param_names: "led_num\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x02,
        name: "off\0",
        doc: "\0", //"Turn a LED off.\0",
        in_signature: "<B\0",
        in_param_names: "led_num\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
];

// - dispatch -----------------------------------------------------------------

#[allow(clippy::too_many_lines)]
impl GreatDispatch for Leds {
    fn dispatch(
        &mut self,
        verb_number: u32,
        arguments: &[u8],
        response_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE],
    ) -> GreatResult<GreatResponse> {
        match verb_number {
            0x00 => {
                // leds::toggle
                let iter = self.toggle(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x01 => {
                // leds::on
                let iter = self.on(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            0x02 => {
                // leds::off
                let iter = self.off(arguments)?;
                let response = iter_to_response(iter, response_buffer);
                Ok(response)
            }
            _verb_number => Err(GreatError::InvalidArgument),
        }
    }
}
