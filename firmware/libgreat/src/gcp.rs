//! Rust implementation of the Great Communications Protocol.
//!
//! More information:
//!
//! * [LibGreat Verb Signatures](https://greatfet.readthedocs.io/en/latest/libgreat_verb_signatures.html)
//! * [LibGreat Class Registry](https://greatfet.readthedocs.io/en/latest/greatfet_classes.html)

pub mod class;
pub mod class_core;
pub use class::*;

// - constants ----------------------------------------------------------------

/// Maximum length of a libgreat command or response
pub const LIBGREAT_MAX_COMMAND_SIZE: usize = 1024;

// - types --------------------------------------------------------------------

use zerocopy::byteorder::{LittleEndian, U32};
use zerocopy::{AsBytes, ByteSlice, FromBytes, FromZeroes, Unaligned};

/// Great Communication Protocol command prelude
#[repr(C)]
#[derive(Debug, FromBytes, FromZeroes, AsBytes, Unaligned)]
pub struct CommandPrelude {
    pub class: U32<LittleEndian>,
    pub verb: U32<LittleEndian>,
}

#[derive(Debug)]
pub struct Command<B: ByteSlice> {
    pub prelude: zerocopy::Ref<B, CommandPrelude>,
    pub arguments: B,
}

impl<B> Command<B>
where
    B: ByteSlice,
{
    pub fn parse(byte_slice: B) -> Option<Command<B>> {
        let (prelude, arguments) = zerocopy::Ref::new_unaligned_from_prefix(byte_slice)?;
        Some(Command { prelude, arguments })
    }

    pub fn class_id(&self) -> ClassId {
        ClassId::from(self.prelude.class)
    }

    pub fn class_number(&self) -> u32 {
        self.prelude.class.into()
    }

    pub fn verb_number(&self) -> u32 {
        self.prelude.verb.get()
    }
}

pub type GreatResponse = core::iter::Take<core::array::IntoIter<u8, LIBGREAT_MAX_COMMAND_SIZE>>;

// - traits -------------------------------------------------------------------

use crate::GreatResult;

pub trait GreatDispatch {
    /// Dispatches a GCP verb.
    ///
    /// # Errors
    ///
    /// Will return [`GreatError`](crate::error::GreatError) on failure.
    fn dispatch(
        &mut self,
        verb_number: u32,
        arguments: &[u8],
        response_buffer: [u8; LIBGREAT_MAX_COMMAND_SIZE],
    ) -> GreatResult<GreatResponse>;
}

// - helpers ------------------------------------------------------------------

/// Squashes an arbitrary Iterator type into a [`GreatResponse`].
///
/// This is not entirely great but it is one solution to the problem
/// of how to dispatch on verbs that return arbiratory iterator types
/// as their response.
pub fn iter_to_response(
    iter: impl Iterator<Item = u8>,
    mut response: [u8; LIBGREAT_MAX_COMMAND_SIZE],
) -> GreatResponse {
    let mut length = 0;
    for (ret, src) in response.iter_mut().zip(iter) {
        *ret = src;
        length += 1;
    }
    response.into_iter().take(length)
}

// - tests --------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use core::array;
    use core::iter;
    use core::slice;

    use zerocopy::byteorder::U16;

    use crate::firmware::BoardInformation;

    use super::*;

    // - fixtures -------------------------------------------------------------

    const COMMAND_NO_ARGS: [u8; 8] = [
        0x01, 0x00, 0x00, 0x00, // class = 1
        0x02, 0x00, 0x00, 0x00, // verb  = 2
    ];
    const COMMAND_READ_BOARD_ID: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, // class = 0 (core)
        0x00, 0x00, 0x00, 0x00, // verb  = 0 (read_board_id)
    ];
    const COMMAND_GET_CLASS_NAME: [u8; 12] = [
        0x00, 0x00, 0x00, 0x00, // class = 0 (core)
        0x08, 0x00, 0x00, 0x00, // verb  = 8 (get_class_name)
        0x01, 0x00, 0x00, 0x00, // arg0: class_number = 1
    ];
    const COMMAND_GET_VERB_DESCRIPTOR: [u8; 17] = [
        0x00, 0x00, 0x00, 0x00, // class = 0 (core)
        0x07, 0x00, 0x00, 0x00, // verb  = 7 (get_verb_descriptor)
        0x00, 0x00, 0x00, 0x00, // arg0: class_number = 0
        0x07, 0x00, 0x00, 0x00, // arg1: verb_number  = 7
        0x01, //                   arg2: descriptor = 1 (in_signature)
    ];
    const COMMAND_SET_UP_ENDPOINTS: [u8; 16] = [
        0x20, 0x01, 0x00, 0x00, // class = 0x0120 (moondancer)
        0x04, 0x00, 0x00, 0x00, // verb  = 4 (set_up_endpoints)
        0x00, //                   arg0: address = 0x00,
        0x40, 0x00, //                   max_packet_size = 64,
        0x00, //                         transfer_type   = 0 (USB_TRANSFER_TYPE_CONTROL)
        0x82, //                   arg1: address = 0x82
        0x00, 0x02, //                   max_packet_size = 512,
        0x02, //                         transfer_type   = 2 (USB_TRANSFER_TYPE_BULK)
    ];

    static CLASS_CORE: Class = Class {
        id: ClassId::core,
        name: "core",
        docs: class_core::CLASS_DOCS,
        verbs: &class_core::VERBS,
    };

    static SUPPORTED_CLASSES: [Class; 1] = [CLASS_CORE];

    pub const BOARD_INFORMATION: BoardInformation = BoardInformation {
        board_id: [0x00, 0x00, 0x00, 0x00],
        version_string: "v2023.0.1\0",
        part_id: [0x30, 0xa, 0x00, 0xa0, 0x5e, 0x4f, 0x60, 0x00],
        serial_number: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe6, 0x67, 0xcc, 0x57, 0x57, 0x53,
            0x6f, 0x30,
        ],
    };

    // - tests ----------------------------------------------------------------

    #[test]
    fn test_enum_class_id() {
        let class_core: ClassId = ClassId::from(0);
        let class_unsupported: ClassId = ClassId::from(0xdeadbeef);
        println!("test_enums: {:?}, {:?}", class_core, class_unsupported,);

        assert_eq!(class_core, ClassId::core);
        assert_eq!(class_unsupported, ClassId::unsupported(0xdeadbeef));
    }

    // - test_parse_* --

    #[test]
    fn test_parse_as_bytes() {
        let prelude: CommandPrelude = CommandPrelude {
            class: 1.into(),
            verb: 2.into(),
        };
        let bytes: &[u8] = prelude.as_bytes();
        println!("test_as_bytes: {:?}", bytes);

        assert_eq!(bytes, [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_parse_from_bytes_no_args() {
        let prelude: CommandPrelude =
            CommandPrelude::read_from(&COMMAND_NO_ARGS[..]).expect("failed parsing command");
        println!("test_from_bytes: {:?}", prelude);

        assert_eq!(prelude.class.get(), 1);
        assert_eq!(prelude.verb.get(), 2);
    }

    #[test]
    fn test_parse_no_args() {
        let command = Command::parse(&COMMAND_NO_ARGS[..]).expect("failed parsing command");
        println!("test_parse_no_args: {:?}", command);

        assert_eq!(command.prelude.class.get(), 1);
        assert_eq!(command.prelude.verb.get(), 2);
    }

    #[test]
    fn test_parse_get_class_name() {
        let command = Command::parse(&COMMAND_GET_CLASS_NAME[..]).expect("failed parsing command");
        println!("test_parse_get_class_name: {:?}", command);

        assert_eq!(command.prelude.class.get(), 0);
        assert_eq!(command.prelude.verb.get(), 8);
    }

    #[test]
    fn test_parse_get_verb_descriptor() {
        #[repr(C)]
        #[derive(Debug, FromBytes, FromZeroes, Unaligned)]
        struct Args {
            class_number: U32<LittleEndian>,
            verb_number: U32<LittleEndian>,
            descriptor: u8,
        }

        let command =
            Command::parse(&COMMAND_GET_VERB_DESCRIPTOR[..]).expect("failed parsing command");
        println!("test_parse_get_verb_descriptor: {:?}", command);

        let args = Args::read_from(command.arguments).unwrap();
        println!("  args: {:?}", args);

        assert_eq!(command.class_id(), ClassId::core);
        assert_eq!(command.verb_number(), 7);
        assert_eq!(args.class_number.get(), 0);
        assert_eq!(args.verb_number.get(), 7);
        assert_eq!(args.descriptor, 1);
    }

    #[test]
    fn test_parse_complex_arguments() {
        #[repr(C)]
        #[derive(Debug, FromBytes, FromZeroes, Unaligned)]
        struct Endpoint {
            address: u8,
            max_packet_size: U16<LittleEndian>,
            transfer_type: u8,
        }
        #[repr(C)]
        #[derive(Debug, FromBytes, FromZeroes, Unaligned)]
        struct Args {
            endpoint: Endpoint,
        }

        let command =
            Command::parse(&COMMAND_SET_UP_ENDPOINTS[..]).expect("failed parsing command");
        println!("test_parse_complex_arguments: {:?}", command);

        assert_eq!(command.class_id(), ClassId::moondancer);
        assert_eq!(command.verb_number(), 4);

        let mut byte_slice = command.arguments;
        while let Some((arg, next)) = zerocopy::Ref::<&[u8], Args>::new_from_prefix(byte_slice) {
            byte_slice = next;
            println!("  arg: {:?}", arg);
        }

        let (arg0, next) = zerocopy::Ref::<&[u8], Args>::new_from_prefix(command.arguments)
            .expect("failed parsing argument");
        assert_eq!(arg0.endpoint.address, 0);
        assert_eq!(arg0.endpoint.max_packet_size.get(), 64);
        assert_eq!(arg0.endpoint.transfer_type, 0);

        let (arg1, next) =
            zerocopy::Ref::<&[u8], Args>::new_from_prefix(next).expect("failed parsing argument");
        assert_eq!(arg1.endpoint.address, 0x82);
        assert_eq!(arg1.endpoint.max_packet_size.get(), 512);
        assert_eq!(arg1.endpoint.transfer_type, 2);

        assert_eq!(next, []);
    }

    // - test_dispatch_* --

    #[test]
    fn test_dispatch_read_board_id() {
        let classes = Classes(&SUPPORTED_CLASSES);
        let mut core = class_core::Core::new(classes, BOARD_INFORMATION);

        let command = Command::parse(&COMMAND_READ_BOARD_ID[..]).expect("failed parsing command");
        println!("\ntest_dispatch_read_board_id: {:?}", command);

        let response_buffer = [0_u8; LIBGREAT_MAX_COMMAND_SIZE];
        let response = core
            .dispatch(command.verb_number(), &command.arguments, response_buffer)
            .expect("failed dispatch");
        println!("  -> {:?}", response);

        let expected: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

        assert_eq!(response.len(), expected.len());
        assert!(response.eq(expected.iter().copied()));
    }

    #[test]
    fn test_dispatch_get_verb_descriptor() {
        let classes = Classes(&SUPPORTED_CLASSES);
        let mut core = class_core::Core::new(classes, BOARD_INFORMATION);

        let command =
            Command::parse(&COMMAND_GET_VERB_DESCRIPTOR[..]).expect("failed parsing command");
        println!("\ntest_dispatch_get_verb_descriptor: {:?}", command);

        let response_buffer = [0_u8; LIBGREAT_MAX_COMMAND_SIZE];
        let response = core
            .dispatch(command.verb_number(), &command.arguments, response_buffer)
            .expect("failed dispatch");
        println!("  -> {:?}", response);

        let expected: [u8; 5] = [60, 73, 73, 73, 0];

        assert_eq!(response.len(), expected.len());
        assert!(response.eq(expected.iter().copied()));
    }

    // - test_introspection --

    fn get_available_classes<'a>() -> impl Iterator<Item = u8> {
        static CLASSES: [u32; 3] = [
            ClassId::core.into_u32(),
            ClassId::firmware.into_u32(),
            ClassId::gpio.into_u32(),
        ];
        CLASSES.iter().flat_map(|class| class.to_le_bytes())
    }

    fn get_available_verbs_core<'a>(verbs: &'a [Verb]) -> impl Iterator<Item = u8> + 'a {
        let iter: slice::Iter<'a, Verb> = verbs.iter();
        let iter = iter.map(|verb| verb.id);
        let iter = iter.flat_map(|verb_number| verb_number.to_le_bytes());
        iter
    }

    #[test]
    fn test_introspection() {
        let classes = get_available_classes();
        let expected = [
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x01, 0x00, 0x00,
        ]
        .iter()
        .copied();
        assert!(classes.eq(expected));

        let verbs = class_core::VERBS;
        let verbs = get_available_verbs_core(&verbs);
        let expected = [
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00,
            0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
            0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00,
        ]
        .iter()
        .copied();
        assert!(verbs.eq(expected));
    }

    // - test_buffer_copy --

    fn get_some_iterator() -> impl Iterator<Item = u8> {
        let mut response: [u8; 32] = [0; 32];
        let iter = get_available_classes();
        let mut length = 0;
        for (ret, src) in response.iter_mut().zip(iter) {
            *ret = src;
            length += 1;
        }
        println!("get_some_iterator: {} bytes - {:?}", length, response);
        response.into_iter().take(length)
    }

    fn iter_to_response(iter: impl Iterator<Item = u8>) -> iter::Take<array::IntoIter<u8, 32>> {
        let mut response: [u8; 32] = [0; 32];
        let mut length = 0;
        for (ret, src) in response.iter_mut().zip(iter) {
            *ret = src;
            length += 1;
        }
        println!("iter_to_response: {} bytes - {:?}", length, response);
        let response: iter::Take<array::IntoIter<u8, 32>> = response.into_iter().take(length);
        response
    }

    fn get_some_other_iterator() -> impl Iterator<Item = u8> {
        let iter = get_available_classes();
        iter_to_response(iter)
    }

    #[test]
    fn test_buffer_copy() {
        println!("\ntest_buffer_copy - get_some_iterator");
        let iter = get_some_iterator();
        for el in iter {
            println!("element: {}", el);
        }

        println!("\ntest_buffer_copy - get_some_other_iterator");
        let iter = get_some_other_iterator();
        for el in iter {
            println!("some other element: {}", el);
        }
    }

    // - test_any --

    use core::any::Any;

    #[derive(Debug, Clone, Copy)]
    struct State {
        value: u32,
    }

    struct Device {}

    impl Device {
        fn new() -> Self {
            Self {}
        }

        fn handle_setup<'a>(&self, some_state: &'a mut dyn Any) -> Option<&'a mut dyn Any> {
            if let Some(state) = some_state.downcast_mut::<State>() {
                println!("handle_setup() state: {:?}", state);
                state.value = 42;
                return Some(some_state);
            }
            Some(some_state)
        }
    }

    #[test]
    fn test_any() {
        let device = Device::new();
        let mut my_state = State { value: 23 };
        println!("my_state: {:?}", my_state);

        let any_state: Option<&mut dyn Any> = device.handle_setup(&mut my_state);
        let any_state = any_state.unwrap();
        println!("any_state: {:?}", any_state);

        if let Some(my_state) = any_state.downcast_mut::<State>() {
            println!("&mut my_state: {:?}", my_state);
        }

        println!("my_state: {:?}", my_state);

        assert_eq!(true, true);
    }
}
