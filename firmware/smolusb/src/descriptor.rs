#![allow(dead_code, unused_imports, unused_variables, unused_mut)] // TODO

use crate::traits::AsByteSliceIterator;
use crate::SmolError;

use heapless::Vec;
use zerocopy::{AsBytes, FromBytes};

use core::iter;
use core::iter::Chain;
use core::marker::PhantomData;
use core::mem::size_of;
use core::slice;

///! USB Descriptors

/// DescriptorType
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum DescriptorType {
    Device = 1,
    Configuration = 2,
    String = 3,
    Interface = 4,
    Endpoint = 5,
    DeviceQualifier = 6,
    OtherSpeedConfiguration = 7,
    InterfacePower = 8,
    OnTheGo = 9,
    Debug = 10,
    InterfaceAssociation = 11,
    Security = 12,
    Key = 13,
    EncryptionType = 14,
    BinaryDeviceObjectStore = 15,
    DeviceCapability = 16,
    WirelessEndpointCompanion = 17,
    SuperSpeedEndpointCompanion = 48,
}

impl TryFrom<u8> for DescriptorType {
    type Error = SmolError;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        let result = match value {
            1 => DescriptorType::Device,
            2 => DescriptorType::Configuration,
            3 => DescriptorType::String,
            4 => DescriptorType::Interface,
            5 => DescriptorType::Endpoint,
            6 => DescriptorType::DeviceQualifier,
            7 => DescriptorType::OtherSpeedConfiguration,
            8 => DescriptorType::InterfacePower,
            9 => DescriptorType::OnTheGo,
            10 => DescriptorType::Debug,
            11 => DescriptorType::InterfaceAssociation,
            12 => DescriptorType::Security,
            13 => DescriptorType::Key,
            14 => DescriptorType::EncryptionType,
            15 => DescriptorType::BinaryDeviceObjectStore,
            16 => DescriptorType::DeviceCapability,
            17 => DescriptorType::WirelessEndpointCompanion,
            48 => DescriptorType::SuperSpeedEndpointCompanion,
            _ => return Err(SmolError::FailedConversion),
        };
        Ok(result)
    }
}

// - DeviceDescriptor ---------------------------------------------------------

/// USB device descriptor
///
/// TODO consider renaming descriptor fields according to LUNA / industry-standard names
#[derive(AsBytes, FromBytes)]
#[repr(C, packed)]
pub struct DeviceDescriptor {
    pub _length: u8,             // 18
    pub _descriptor_type: u8,    // 1 = Device
    pub descriptor_version: u16, // aka bcdUSB
    pub device_class: u8,
    pub device_subclass: u8,
    pub device_protocol: u8,
    pub max_packet_size: u8,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_version_number: u16,
    pub manufacturer_string_index: u8,
    pub product_string_index: u8,
    pub serial_string_index: u8,
    pub num_configurations: u8,
}

impl AsByteSliceIterator for DeviceDescriptor {}

impl DeviceDescriptor {
    pub const fn new() -> Self {
        Self {
            _length: size_of::<Self>() as u8,
            _descriptor_type: DescriptorType::Device as u8,
            descriptor_version: 0x0200,
            device_class: 0,
            device_subclass: 0,
            device_protocol: 0,
            max_packet_size: 0,
            vendor_id: 0,
            product_id: 0,
            device_version_number: 0,
            manufacturer_string_index: 0,
            product_string_index: 0,
            serial_string_index: 0,
            num_configurations: 0,
        }
    }
}

impl Default for DeviceDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// - DeviceQualifierDescriptor ------------------------------------------------

/// USB device qualifier descriptor
#[derive(AsBytes, FromBytes)]
#[repr(C, packed)]
pub struct DeviceQualifierDescriptor {
    pub _length: u8,          // 10
    pub _descriptor_type: u8, // 6 = DeviceQualifier
    pub descriptor_version: u16,
    pub device_class: u8,
    pub device_subclass: u8,
    pub device_protocol: u8,
    pub max_packet_size: u8,
    pub num_configurations: u8,
    pub reserved: u8,
}

impl AsByteSliceIterator for DeviceQualifierDescriptor {}

impl DeviceQualifierDescriptor {
    pub const fn new() -> Self {
        Self {
            _length: size_of::<Self>() as u8,
            _descriptor_type: DescriptorType::DeviceQualifier as u8,
            descriptor_version: 0,
            device_class: 0,
            device_subclass: 0,
            device_protocol: 0,
            max_packet_size: 0,
            num_configurations: 0,
            reserved: 0,
        }
    }
}

impl Default for DeviceQualifierDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// - ConfigurationDescriptor --------------------------------------------------

/// USB configuration descriptor
#[derive(Clone, Copy)]
pub struct ConfigurationDescriptor<'a> {
    head: ConfigurationDescriptorHeader,
    tail: &'a [InterfaceDescriptor<'a>],
}

impl<'a> ConfigurationDescriptor<'a> {
    pub const fn new(
        mut head: ConfigurationDescriptorHeader,
        tail: &'a [InterfaceDescriptor],
    ) -> Self {
        head._length = size_of::<ConfigurationDescriptorHeader>() as u8;
        head._num_interfaces = tail.len() as u8;

        Self { head, tail }
    }

    /// Calculate and update the descriptor total length field
    pub fn set_total_length(&mut self) -> usize {
        let total_length = self.iter().count();
        self.head._total_length = total_length as u16;
        total_length
    }

    pub fn iter(&self) -> ConfigurationDescriptorIterator {
        ConfigurationDescriptorIterator::new(self)
    }
}

/// USB configuration descriptor header
#[derive(AsBytes, FromBytes, Clone, Copy)]
#[repr(C, packed)]
pub struct ConfigurationDescriptorHeader {
    pub _length: u8,         // 9
    pub descriptor_type: u8, // 2 = Configuration, 3 = OtherSpeedConfiguration TODO
    pub _total_length: u16,
    pub _num_interfaces: u8,
    pub configuration_value: u8,
    pub configuration_string_index: u8,
    pub attributes: u8,
    pub max_power: u8,
}

impl AsByteSliceIterator for ConfigurationDescriptorHeader {}

impl ConfigurationDescriptorHeader {
    pub const fn new() -> Self {
        Self {
            _length: size_of::<Self>() as u8,
            descriptor_type: DescriptorType::Configuration as u8,
            _total_length: 0,
            _num_interfaces: 0,
            configuration_value: 0,
            configuration_string_index: 0,
            attributes: 0,
            max_power: 0,
        }
    }
}

/// USB configuration descriptor iterator
pub struct ConfigurationDescriptorIterator<'a> {
    chain: iter::Chain<slice::Iter<'a, u8>, ConfigurationDescriptorTailIterator<'a>>,
}

impl<'a> ConfigurationDescriptorIterator<'a> {
    pub fn new(descriptor: &'a ConfigurationDescriptor) -> Self {
        let head_iter: slice::Iter<'a, u8> = descriptor.head.as_iter();
        let tail_iter: ConfigurationDescriptorTailIterator = descriptor
            .tail
            .iter()
            .flat_map(&|x: &'a InterfaceDescriptor| x.iter());
        let chain: iter::Chain<slice::Iter<'a, u8>, ConfigurationDescriptorTailIterator<'a>> =
            head_iter.chain(tail_iter);

        Self { chain }
    }
}

impl<'a> Iterator for ConfigurationDescriptorIterator<'a> {
    type Item = &'a u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.chain.next()
    }
}

// type aliases for sanity
type InterfaceDescriptorIterator<'a> =
    CompositeIterator<'a, InterfaceDescriptorHeader, EndpointDescriptor>;
type ConfigurationDescriptorTailIterator<'a> = iter::FlatMap<
    slice::Iter<'a, InterfaceDescriptor<'a>>,
    InterfaceDescriptorIterator<'a>,
    &'a dyn Fn(&'a InterfaceDescriptor<'a>) -> InterfaceDescriptorIterator<'a>,
>;

// - InterfaceDescriptor ------------------------------------------------------

/// USB interface descriptor
pub struct InterfaceDescriptor<'a> {
    head: InterfaceDescriptorHeader,
    tail: &'a [EndpointDescriptor],
}

impl<'a> InterfaceDescriptor<'a> {
    pub const fn new(mut head: InterfaceDescriptorHeader, tail: &'a [EndpointDescriptor]) -> Self {
        head._length = size_of::<InterfaceDescriptorHeader>() as u8;
        head._num_endpoints = tail.len() as u8;
        Self { head, tail }
    }

    pub fn iter(&'a self) -> CompositeIterator<'a, InterfaceDescriptorHeader, EndpointDescriptor> {
        let iter = CompositeIterator::new(&self.head, self.tail);
        iter
    }
}

/// USB interface descriptor header
#[derive(AsBytes, FromBytes)]
#[repr(C, packed)]
pub struct InterfaceDescriptorHeader {
    pub _length: u8,          // 9
    pub _descriptor_type: u8, // 4 = Interface
    pub interface_number: u8,
    pub alternate_setting: u8,
    pub _num_endpoints: u8,
    pub interface_class: u8,
    pub interface_subclass: u8,
    pub interface_protocol: u8,
    pub interface_string_index: u8,
}

impl AsByteSliceIterator for InterfaceDescriptorHeader {}

impl InterfaceDescriptorHeader {
    pub const fn new() -> Self {
        Self {
            _length: size_of::<Self>() as u8,
            _descriptor_type: DescriptorType::Interface as u8,
            interface_number: 0,
            alternate_setting: 0,
            _num_endpoints: 0,
            interface_class: 0,
            interface_subclass: 0,
            interface_protocol: 0,
            interface_string_index: 0,
        }
    }
}

// - EndpointDescriptor -------------------------------------------------------

/// USB endpoint descriptor
#[derive(AsBytes, FromBytes)]
#[repr(C, packed)]
pub struct EndpointDescriptor {
    pub _length: u8,          // 7
    pub _descriptor_type: u8, // 5 = Endpoint
    pub endpoint_address: u8,
    pub attributes: u8,
    pub max_packet_size: u16,
    pub interval: u8,
}

impl AsByteSliceIterator for EndpointDescriptor {}

impl EndpointDescriptor {
    pub const fn new() -> Self {
        Self {
            _length: size_of::<Self>() as u8,
            _descriptor_type: DescriptorType::Endpoint as u8,
            endpoint_address: 0,
            attributes: 0,
            max_packet_size: 0,
            interval: 0,
        }
    }
}

impl Default for EndpointDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// - LanguageId ---------------------------------------------------------------

/// USB string descriptor language id
#[derive(AsBytes, Copy, Clone, Debug)]
#[repr(u16)]
pub enum LanguageId {
    EnglishUnitedStates = 0x0409,
    EnglishUnitedKingdom = 0x0809,
    EnglishCanadian = 0x1009,
    EnglishSouthAfrica = 0x1c09,
}

impl AsByteSliceIterator for LanguageId {}

// - StringDescriptorZero -----------------------------------------------------

/// USB string zero descriptor
pub struct StringDescriptorZero<'a> {
    head: StringDescriptorHeader,
    tail: &'a [LanguageId],
}

impl<'a> StringDescriptorZero<'a> {
    pub const fn new(language_ids: &'a [LanguageId]) -> Self {
        let head_length = size_of::<StringDescriptorHeader>();
        let tail_length = language_ids.len() * size_of::<LanguageId>();
        Self {
            head: StringDescriptorHeader {
                _length: (head_length + tail_length) as u8,
                _descriptor_type: DescriptorType::String as u8,
            },
            tail: language_ids,
        }
    }

    pub fn iter(&'a self) -> CompositeIterator<'a, StringDescriptorHeader, LanguageId> {
        let iter = CompositeIterator::new(&self.head, self.tail);
        iter
    }
}

/// USB string zero descriptor header
#[derive(AsBytes, FromBytes, Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct StringDescriptorHeader {
    pub _length: u8,
    pub _descriptor_type: u8, // 3 = String
}

impl StringDescriptorHeader {
    pub const fn new() -> Self {
        Self {
            _length: 0,
            _descriptor_type: DescriptorType::String as u8,
        }
    }
}

impl AsByteSliceIterator for StringDescriptorHeader {}

// - StringDescriptor ---------------------------------------------------------

/// USB String Descriptor
#[derive(Clone, Copy)]
pub struct StringDescriptor<'a> {
    head: StringDescriptorHeader,
    tail: &'a str,
}

impl<'a> StringDescriptor<'a> {
    pub const fn new(string: &'a str) -> Self {
        let head_length = size_of::<Self>();
        // TODO this may not be accurate
        let tail_length = string.len() * 2;

        Self {
            head: StringDescriptorHeader {
                _length: (head_length + tail_length) as u8,
                _descriptor_type: DescriptorType::String as u8,
            },
            tail: string,
        }
    }
}

impl<'a> StringDescriptor<'a> {
    /// Calculate and update the descriptor length field
    pub fn set_length(&mut self) -> usize {
        let length = self.iter().count();
        self.head._length = length as u8;
        length
    }

    /// Returns an iterator to the descriptor
    pub fn iter(&'a self) -> StringDescriptorIterator<'a> {
        let head_iter: slice::Iter<'a, u8> = self.head.as_iter();

        // TODO USB string descriptors can be a maximum of 126 characters
        let tail_iter: Utf16ByteIterator = Utf16ByteIterator::new(self.tail.encode_utf16());

        head_iter.cloned().chain(tail_iter)
    }
}

pub type StringDescriptorIterator<'a> =
    iter::Chain<iter::Cloned<slice::Iter<'a, u8>>, Utf16ByteIterator<'a>>;

#[allow(dead_code)]
fn static_test_string_descriptor() {
    let descriptor = StringDescriptor::new("TRI-FIFO Example");
    for byte in descriptor.iter() {
        let _byte: u8 = byte;
    }
}

// - Utf16ByteIterator --------------------------------------------------------

#[derive(Clone)]
pub struct Utf16ByteIterator<'a> {
    encode_utf16: core::str::EncodeUtf16<'a>,
    byte: Option<u8>,
}

impl<'a> Utf16ByteIterator<'a> {
    pub fn new(encode_utf16: core::str::EncodeUtf16<'a>) -> Self {
        Self {
            encode_utf16,
            byte: None,
        }
    }
}

impl<'a> Iterator for Utf16ByteIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match self.byte {
            Some(_) => self.byte.take(),
            None => match self.encode_utf16.next() {
                Some(unicode_char) => {
                    let bytes: [u8; 2] = unicode_char.to_le_bytes();
                    self.byte = Some(bytes[1]);
                    Some(bytes[0])
                }
                None => None,
            },
        }
    }
}

// - CompositeIterator --------------------------------------------------------

type HeadIterator<'a> = slice::Iter<'a, u8>;
type TailIterator<'a, T> = iter::FlatMap<
    slice::Iter<'a, T>,
    slice::Iter<'a, u8>,
    &'a dyn Fn(&'a T) -> slice::Iter<'a, u8>,
>;
type CompositeChain<'a, T> = iter::Chain<slice::Iter<'a, u8>, TailIterator<'a, T>>;

pub struct CompositeIterator<'a, H, T> {
    chain: CompositeChain<'a, T>,
    _marker: PhantomData<H>,
}

impl<'a, H, T> CompositeIterator<'a, H, T>
where
    H: AsByteSliceIterator + 'a,
    T: AsByteSliceIterator + 'a,
{
    pub fn new(head: &'a H, tail: &'a [T]) -> Self {
        let head_iter: HeadIterator<'a> = head.as_iter();
        let tail_iter: TailIterator<'a, T> = tail.iter().flat_map(&|x: &'a T| x.as_iter());
        let chain: CompositeChain<'a, T> = head_iter.chain(tail_iter);
        Self {
            chain,
            _marker: PhantomData,
        }
    }
}

impl<'a, H, T> Iterator for CompositeIterator<'a, H, T> {
    type Item = &'a u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.chain.next()
    }
}
