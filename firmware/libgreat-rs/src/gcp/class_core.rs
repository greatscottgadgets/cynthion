use log::trace;
use zerocopy::{FromBytes, LittleEndian, Unaligned, U32};

use crate::error::{GreatError, GreatResult};
use crate::firmware::BoardInformation;
use crate::gcp::{self, Classes};

use super::{Verb, VerbDescriptor};

pub static CLASS: gcp::Class = gcp::Class {
    id: gcp::ClassId::core,
    name: "core",
    docs: CLASS_DOCS,
    verbs: &VERBS,
};

pub static CLASS_DOCS: &str = "Core API\0"; // used to query information about the device, and perform a few standard functions.\0";

/// Fields are `"\0"`  where C implementation has `""`
/// Fields are `"*\0"` where C implementation has `NULL`
pub static VERBS: [Verb; 10] = [
    Verb {
        id: 0x0,
        name: "read_board_id\0",
        doc: "*\0",
        in_signature: "*\0",
        in_param_names: "*\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x1,
        name: "read_version_string\0",
        doc: "*\0",
        in_signature: "*\0",
        in_param_names: "*\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x2,
        name: "read_part_id\0",
        doc: "*\0",
        in_signature: "*\0",
        in_param_names: "*\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x3,
        name: "read_serial_number\0",
        doc: "*\0",
        in_signature: "*\0",
        in_param_names: "*\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },
    // - api introspection --
    Verb {
        id: 0x4,
        name: "get_available_classes\0",
        doc: "*\0",
        in_signature: "*\0",
        in_param_names: "*\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x5,
        name: "get_available_verbs\0",
        doc: "*\0",
        in_signature: "<I\0",
        in_param_names: "class_number\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x6,
        name: "get_verb_name\0",
        doc: "*\0",
        in_signature: "<II\0",
        in_param_names: "class_number, verb_number\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x7,
        name: "get_verb_descriptor\0",
        doc: "*\0",
        in_signature: "<III\0",
        in_param_names: "class_number, verb_number, descriptor_number\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x8,
        name: "get_class_name\0",
        doc: "*\0",
        in_signature: "<I\0",
        in_param_names: "class_number\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },
    Verb {
        id: 0x9,
        name: "get_class_docs\0",
        doc: "*\0",
        in_signature: "<I\0",
        in_param_names: "class_number\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },
    /*Verb {
        id: 0x20,
        name: "*\0", // request_reset
        doc: "*\0",
        in_signature: "*\0",
        in_param_names: "*\0",
        out_signature: "*\0",
        out_param_names: "*\0",
    },*/
];

// - Core ---------------------------------------------------------------------

pub struct Core {
    classes: Classes,
    board_information: BoardInformation,
}

impl Core {
    pub fn new(classes: Classes, board_information: BoardInformation) -> Self {
        Self {
            classes,
            board_information,
        }
    }
}

// - verb implementations: board ----------------------------------------------

impl Core {
    pub fn read_board_id(&self, _arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        let board_id = self.board_information.board_id;
        trace!("  sending board id: {:?}", board_id);
        Ok(board_id.into_iter())
    }

    pub fn read_version_string(&self, _arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        let version_string = self.board_information.version_string;
        trace!("  sending version string: {:?}", version_string);
        Ok(version_string.as_bytes().into_iter().copied())
    }

    pub fn read_part_id(&self, _arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        let part_id = self.board_information.part_id;
        trace!("  sending part id: {:?}", part_id);
        Ok(part_id.into_iter())
    }

    pub fn read_serial_number(&self, _arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        let serial_number = self.board_information.serial_number;
        trace!("  sending serial number: {:?}", serial_number);
        Ok(serial_number.into_iter())
    }
}

// - verb implementations: introspection --------------------------------------

impl Core {
    pub fn get_available_classes(
        &self,
        _arguments: &[u8],
    ) -> GreatResult<impl Iterator<Item = u8>> {
        let classes = self
            .classes
            .iter()
            .flat_map(|class| class.id.into_u32().to_le_bytes());
        Ok(classes)
    }

    pub fn get_available_verbs(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            class_number: U32<LittleEndian>,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let class_id = args.class_number.into();
        let class = self
            .classes
            .class(class_id)
            .ok_or(GreatError::InvalidArgument)?;
        let verbs = class.verbs.iter().flat_map(|verb| verb.id.to_le_bytes());
        Ok(verbs)
    }

    pub fn get_verb_name(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            class_number: U32<LittleEndian>,
            verb_number: U32<LittleEndian>,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let class_id = args.class_number.into();
        let class = self
            .classes
            .class(class_id)
            .ok_or(GreatError::InvalidArgument)?;
        let verb = class
            .verb(args.verb_number.into())
            .ok_or(GreatError::InvalidArgument)?;
        Ok(verb.name.as_bytes().into_iter().copied())
    }

    pub fn get_verb_descriptor(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(Debug, FromBytes, Unaligned)]
        struct Args {
            class_number: U32<LittleEndian>,
            verb_number: U32<LittleEndian>,
            descriptor: u8,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let class_id = args.class_number.into();
        let class = self
            .classes
            .class(class_id)
            .ok_or(GreatError::InvalidArgument)?;
        let verb = class
            .verb(args.verb_number.into())
            .ok_or(GreatError::InvalidArgument)?;
        match args.descriptor.into() {
            VerbDescriptor::InSignature => Ok(verb.in_signature.as_bytes().into_iter().copied()),
            VerbDescriptor::InParamNames => Ok(verb.in_param_names.as_bytes().into_iter().copied()),
            VerbDescriptor::OutSignature => Ok(verb.out_signature.as_bytes().into_iter().copied()),
            VerbDescriptor::OutParamNames => {
                Ok(verb.out_param_names.as_bytes().into_iter().copied())
            }
            VerbDescriptor::Doc => Ok(verb.doc.as_bytes().into_iter().copied()),
            VerbDescriptor::Unknown(_value) => Err(GreatError::InvalidRequestDescriptor),
        }
    }

    pub fn get_class_name(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        trace!("  get_class_name: {:?}", arguments);
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            class_number: U32<LittleEndian>,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let class_id = args.class_number.into();
        let class = self
            .classes
            .class(class_id)
            .ok_or(GreatError::InvalidArgument)?;
        Ok(class.name.as_bytes().iter().copied())
    }

    pub fn get_class_docs(&self, arguments: &[u8]) -> GreatResult<impl Iterator<Item = u8>> {
        #[repr(C)]
        #[derive(FromBytes, Unaligned)]
        struct Args {
            class_number: U32<LittleEndian>,
        }
        let args = Args::read_from(arguments).ok_or(GreatError::InvalidArgument)?;
        let class_id = args.class_number.into();
        let class = self
            .classes
            .class(class_id)
            .ok_or(GreatError::InvalidArgument)?;
        Ok(class.docs.as_bytes().into_iter().copied())
    }
}

// - dispatch -----------------------------------------------------------------

use crate::gcp::{iter_to_response, GreatResponse, LIBGREAT_MAX_COMMAND_SIZE};

impl Core {
    pub fn dispatch(
        &self,
        verb_number: u32,
        arguments: &[u8],
        response_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE],
    ) -> GreatResult<GreatResponse> {
        match verb_number {
            0x0 => {
                // core::read_board_id
                let iter = self.read_board_id(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x1 => {
                // core::read_version_string
                let iter = self.read_version_string(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x2 => {
                // core::read_part_id
                let iter = self.read_part_id(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x3 => {
                // core::read_serial_number
                let iter = self.read_serial_number(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x4 => {
                // core::get_available_classes
                let iter = self.get_available_classes(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x5 => {
                // core::get_available_verbs
                let iter = self.get_available_verbs(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x6 => {
                // core::get_verb_name
                let iter = self.get_verb_name(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x7 => {
                // core::get_verb_descriptor
                let iter = self.get_verb_descriptor(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x8 => {
                // core::get_class_name
                let iter = self.get_class_name(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }
            0x9 => {
                // core::get_class_docs
                let iter = self.get_class_docs(arguments)?;
                let response = unsafe { iter_to_response(iter, response_buffer) };
                Ok(response)
            }

            _verb_number => Err(GreatError::InvalidArgument),
        }
    }
}
