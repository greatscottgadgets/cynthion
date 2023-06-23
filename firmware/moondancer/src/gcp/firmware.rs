#![allow(dead_code, unused_imports, unused_variables)] // TODO

use libgreat::error::{GreatError, GreatResult};
use libgreat::gcp::{self, Verb};

use log::{debug, error};
use zerocopy::{AsBytes, BigEndian, FromBytes, LittleEndian, Unaligned, U32};

use core::any::Any;
use core::slice;

pub static CLASS: gcp::Class = gcp::Class {
    id: gcp::ClassId::firmware,
    name: "firmware",
    docs: CLASS_DOCS,
    verbs: &VERBS,
};

pub static CLASS_DOCS: &str = "Common API for updating firmware on a libgreat device.\0";

/// Fields are `"\0"`  where C implementation has `""`
/// Fields are `"*\0"` where C implementation has `NULL`
pub static VERBS: [Verb; 5] = [
    Verb {
        id: 0x0,
        name: "initialize\0",
        doc: "\0", //"Prepare the board to have its firmware programmed.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "<II\0",
        out_param_names: "page_size, total_size\0",
    },
    Verb {
        id: 0x1,
        name: "full_erase\0",
        doc: "\0", //"Erase the entire firmware flash chip.\0",
        in_signature: "\0",
        in_param_names: "*\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x2,
        name: "page_erase\0",
        doc: "\0", //"Erase the page with the given address on the firmware flash chip.\0",
        in_signature: "<I\0",
        in_param_names: "address\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x3,
        name: "write_page\0",
        doc: "\0", //"Write the provided data to a single firmware flash page.\0",
        in_signature: "<I*X\0",
        in_param_names: "address, data\0",
        out_signature: "\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x4,
        name: "read_page\0",
        doc: "\0", //"Return the content of the flash page at the given address.\0",
        in_signature: "<I\0",
        in_param_names: "address\0",
        out_signature: "<*X\0",
        out_param_names: "data\0",
    },
];

// - verb implementations -----------------------------------------------------

pub fn initialize<'a>(
    arguments: &[u8],
    _context: &'a dyn Any,
) -> GreatResult<impl Iterator<Item = u8> + 'a> {
    let page_size: u32 = 256;
    let total_size: u32 = 256 * 8192;
    let response = page_size
        .to_le_bytes()
        .into_iter()
        .chain(total_size.to_le_bytes().into_iter());
    Ok(response)
}

pub fn full_erase<'a>(
    arguments: &[u8],
    _context: &'a dyn Any,
) -> GreatResult<impl Iterator<Item = u8> + 'a> {
    Ok([].into_iter())
}

pub fn page_erase<'a>(
    arguments: &[u8],
    _context: &'a dyn Any,
) -> GreatResult<impl Iterator<Item = u8> + 'a> {
    #[repr(C)]
    #[derive(FromBytes, Unaligned)]
    struct Args {
        address: U32<LittleEndian>,
    }
    let _args = Args::read_from(arguments).ok_or(GreatError::BadMessage)?;
    Ok([].into_iter())
}

pub fn write_page<'a>(
    arguments: &[u8],
    _context: &'a dyn Any,
) -> GreatResult<impl Iterator<Item = u8> + 'a> {
    struct Args<B: zerocopy::ByteSlice> {
        address: zerocopy::LayoutVerified<B, U32<LittleEndian>>,
        data: B,
    }
    let (address, data) = zerocopy::LayoutVerified::new_unaligned_from_prefix(arguments)
        .ok_or(GreatError::BadMessage)?;
    let _args = Args { address, data };
    Ok([].into_iter())
}

pub fn read_page<'a>(
    arguments: &[u8],
    _context: &'a dyn Any,
) -> GreatResult<impl Iterator<Item = u8> + 'a> {
    #[repr(C)]
    #[derive(FromBytes, Unaligned)]
    struct Args {
        address: U32<LittleEndian>,
    }
    let _args = Args::read_from(arguments).ok_or(GreatError::BadMessage)?;
    let data: [u8; 8] = [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
    Ok(data.into_iter())
}

// - dispatch -----------------------------------------------------------------

use libgreat::gcp::{iter_to_response, GcpResponse, GCP_MAX_RESPONSE_LENGTH};

use core::{array, iter};

pub fn dispatch(
    verb_number: u32,
    arguments: &[u8],
    response_buffer: [u8; GCP_MAX_RESPONSE_LENGTH],
) -> GreatResult<GcpResponse> {
    let no_context: Option<u8> = None;

    match verb_number {
        0x0 => {
            // firmware::initialize
            let iter = initialize(arguments, &no_context)?;
            let response = unsafe { iter_to_response(iter, response_buffer) };
            Ok(response)
        }
        0x1 => {
            // firmware::full_erase
            let iter = full_erase(arguments, &no_context)?;
            let response = unsafe { iter_to_response(iter, response_buffer) };
            Ok(response)
        }
        0x2 => {
            // firmware::page_erase
            let iter = page_erase(arguments, &no_context)?;
            let response = unsafe { iter_to_response(iter, response_buffer) };
            Ok(response)
        }
        0x3 => {
            // firmware::write_page
            let iter = write_page(arguments, &no_context)?;
            let response = unsafe { iter_to_response(iter, response_buffer) };
            Ok(response)
        }
        0x4 => {
            // firmware::read_page
            let iter = read_page(arguments, &no_context)?;
            let response = unsafe { iter_to_response(iter, response_buffer) };
            Ok(response)
        }

        verb_number => Err(GreatError::GcpVerbNotFound(
            gcp::class::ClassId::firmware,
            verb_number,
        )),
    }
}
