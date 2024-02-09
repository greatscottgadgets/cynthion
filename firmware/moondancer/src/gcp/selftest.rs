use libgreat::error::{GreatError, GreatResult};
use libgreat::gcp::{self, Verb};

use log::debug;
use zerocopy::{FromBytes, LittleEndian, Unaligned, U32};

use core::any::Any;

pub static CLASS: gcp::Class = gcp::Class {
    id: gcp::ClassId::selftest,
    name: "selftest",
    docs: CLASS_DOCS,
    verbs: &VERBS,
};

pub static CLASS_DOCS: &str = "Provides functionality for a Cynthion to self-test itself.\0";

/// Fields are `"\0"`  where C implementation has `""`
/// Fields are `"*\0"` where C implementation has `NULL`
pub static VERBS: [Verb; 1] = [Verb {
    id: 0x10,
    name: "test_error_return_code\0",
    doc: "\0", // "Returns the string 'ok' if code is 0, otherwise an error with the given code.\0",
    in_signature: "<I\0",
    in_param_names: "code\0",
    out_signature: "<S\0",
    out_param_names: "result\0",
}];

// - verb implementations -----------------------------------------------------

pub fn test_error_return_code<'a>(
    arguments: &[u8],
    _context: &'a dyn Any,
) -> GreatResult<impl Iterator<Item = u8> + 'a> {
    #[repr(C)]
    #[derive(FromBytes, Unaligned)]
    struct Args {
        code: U32<LittleEndian>,
    }
    let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;

    match args.code.into() {
        0_u32 => {
            let s = "ok";
            debug!("  test_error_return_code -> 0 -> Ok('ok')");
            Ok(s.as_bytes().iter().copied())
        }
        code => {
            let code: GreatError = unsafe { core::mem::transmute(code) };
            debug!("  test_error_return_code -> {} -> Err({})", args.code, code);
            Err(code)
        }
    }
}

// - dispatch -----------------------------------------------------------------

use libgreat::gcp::{iter_to_response, GreatResponse, LIBGREAT_MAX_COMMAND_SIZE};

pub fn dispatch(
    verb_number: u32,
    arguments: &[u8],
    response_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE],
) -> GreatResult<GreatResponse> {
    let no_context: Option<u8> = None;

    match verb_number {
        0x10 => {
            // selftest::test_error_return_code
            let iter = test_error_return_code(arguments, &no_context)?;
            let response = iter_to_response(iter, response_buffer);
            Ok(response)
        }

        _verb_number => Err(GreatError::InvalidArgument),
    }
}
