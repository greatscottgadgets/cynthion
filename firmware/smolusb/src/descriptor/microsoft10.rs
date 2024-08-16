//! Microsoft OS 1.0 USB Descriptors

use core::iter;
use core::marker::PhantomData;
use core::mem::size_of;
use core::slice;

use zerocopy::{AsBytes, FromBytes, FromZeroes};

use crate::descriptor::{CompositeIterator, DescriptorType, StringDescriptorHeader};
use crate::traits::AsByteSliceIterator;

// - Constants ----------------------------------------------------------------

/// Vendor Requests
#[non_exhaustive]
pub struct VendorRequest;

#[allow(non_upper_case_globals)]
impl VendorRequest {
    pub const Microsoft: u8 = 0xee;
}

/// Vendor Indices
#[repr(u16)]
#[derive(Debug, PartialEq)]
pub enum VendorIndex {
    CompatibleIdFeatureDescriptor = 0x0004,
    ExtendedPropertiesFeatureDescriptor = 0x0005,
    Unknown(u16),
}

impl From<u16> for VendorIndex {
    fn from(value: u16) -> Self {
        match value {
            0x0004 => VendorIndex::CompatibleIdFeatureDescriptor,
            0x0005 => VendorIndex::ExtendedPropertiesFeatureDescriptor,
            _ => VendorIndex::Unknown(value),
        }
    }
}

// - Descriptors --------------------------------------------------------------

/// Microsoft OS 1.0 Descriptors
pub struct Descriptors<'a> {
    pub string_descriptor: StringDescriptor<'a>,
    pub compat_id_feature_descriptor: CompatibleIdFeatureDescriptor<'a>,
    pub extended_properties_feature_descriptor: ExtendedPropertiesFeatureDescriptor,
}

// - StringDescriptor ---------------------------------------------------------

/// Microsoft OS 1.0 String Descriptor Header
#[derive(Clone, Copy)]
pub struct StringDescriptor<'a> {
    pub head: StringDescriptorHeader,
    pub tail: [u8; 16],
    _marker: PhantomData<&'a ()>,
}

impl<'a> StringDescriptor<'a> {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new(vendor_code: u8) -> Self {
        Self {
            head: StringDescriptorHeader {
                bLength: 18,
                bDescriptorType: DescriptorType::String as u8,
            },
            #[rustfmt::skip]
            tail: [
                // Signature: "MSFT100"
                b'M', 0x00, b'S', 0x00, b'F', 0x00, b'T', 0x00, b'1', 0x00, b'0', 0x00, b'0', 0x00,
                // Vendor Code
                vendor_code,
                // Padding
                0x0,
            ],
            _marker: PhantomData,
        }
    }
}

impl<'a> StringDescriptor<'a> {
    /// Returns an iterator to the descriptor
    #[allow(clippy::cloned_instead_of_copied)]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&'a self) -> StringDescriptorIterator<'a> {
        let head_iter: slice::Iter<'a, u8> = self.head.as_iter();
        let tail_iter: slice::Iter<'a, u8> = self.tail.iter();
        head_iter.cloned().chain(tail_iter.cloned())
    }
}

pub type StringDescriptorIterator<'a> =
    iter::Chain<iter::Cloned<slice::Iter<'a, u8>>, iter::Cloned<slice::Iter<'a, u8>>>;

// - CompatibleIdFeatureDescriptor --------------------------------------------

/// Microsoft OS 1.0 Compatible ID Feature Descriptor header
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy)]
#[repr(C, packed)]
pub struct CompatibleIdFeatureDescriptorHeader {
    pub dwLength: u32,
    pub bcdVersion: u16, // 0x0100
    pub wIndex: u16,     // 0x0004 Compatible ID Feature Descriptor
    pub bCount: u8,
    pub aReserved: [u8; 7],
}

impl CompatibleIdFeatureDescriptorHeader {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            dwLength: 0,
            bcdVersion: 0x0100, // v1.0
            wIndex: 0x0004,     // Compatible ID Feature Descriptor
            bCount: 0,
            aReserved: [0; 7],
        }
    }
}

impl AsByteSliceIterator for CompatibleIdFeatureDescriptorHeader {}

/// Microsoft OS 1.0 Compatible ID Feature Descriptor function section
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy)]
#[repr(C, packed)]
pub struct CompatibleIdFeatureDescriptorFunction {
    pub bFirstInterfaceNumber: u8,
    pub bReserved: u8,
    pub aCompatibleId: [u8; 8],
    pub aSubCompatibleId: [u8; 8],
    pub aReserved: [u8; 6],
}

impl CompatibleIdFeatureDescriptorFunction {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bFirstInterfaceNumber: 0,
            bReserved: 0x01, // TODO does this _need_ to be 0x01 ???
            aCompatibleId: [0; 8],
            aSubCompatibleId: [0; 8],
            aReserved: [0; 6],
        }
    }
}

impl AsByteSliceIterator for CompatibleIdFeatureDescriptorFunction {}

/// Microsoft OS 1.0 Compatible ID Feature Descriptor
pub struct CompatibleIdFeatureDescriptor<'a> {
    pub head: CompatibleIdFeatureDescriptorHeader,
    pub tail: &'a [CompatibleIdFeatureDescriptorFunction],
}

impl<'a> CompatibleIdFeatureDescriptor<'a> {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new(tail: &'a [CompatibleIdFeatureDescriptorFunction]) -> Self {
        Self {
            head: CompatibleIdFeatureDescriptorHeader {
                dwLength: (size_of::<CompatibleIdFeatureDescriptorHeader>()
                    + (size_of::<CompatibleIdFeatureDescriptorFunction>() * tail.len()))
                    as u32,
                bCount: tail.len() as u8,
                ..CompatibleIdFeatureDescriptorHeader::new()
            },
            tail,
        }
    }
}

impl<'a> CompatibleIdFeatureDescriptor<'a> {
    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(
        &'a self,
    ) -> CompositeIterator<
        'a,
        CompatibleIdFeatureDescriptorHeader,
        CompatibleIdFeatureDescriptorFunction,
    > {
        let iter = CompositeIterator::new(&self.head, self.tail);
        iter
    }
}

// - ExtendedPropertiesFeatureDescriptor --------------------------------------

/// Microsoft OS 1.0 Extended Properties Feature Descriptor
///
/// FIXME this is hardcoded for a single use-case but needs to be generic
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy)]
#[repr(C, packed)]
pub struct ExtendedPropertiesFeatureDescriptor {
    // Header
    pub dwLength: u32,
    pub bcdVersion: u16, // 0x0100
    pub wIndex: u16,     // 0x0005 Extended Properties Feature Descriptor
    pub wCount: u16,     // 0x01

    // Property Section
    pub dwSize: u32,
    pub dwPropertyDataType: u32,
    pub wPropertyNameLength: u16,
    pub bPropertyName: [u8; 40],
    pub dwPropertyDataLength: u32,
    pub bPropertyData: [u8; 78],
}

impl ExtendedPropertiesFeatureDescriptor {
    #[must_use]
    #[rustfmt::skip]
    pub const fn new() -> Self {
        Self {
            // Header
            dwLength: 0x0000_008E,              // 142 bytes
            bcdVersion: 0x0100,                 // v1.0
            wIndex: 0x0005,                     // Extended Properties Feature Descriptor
            wCount: 0x0001,                     // Number of sections
            // Property Section
            dwSize: 0x0000_0084,                // 132 bytes
            dwPropertyDataType: 0x0000_0001,    // 1 = Unicode REG_SZ
            wPropertyNameLength: 0x0028,        // 40 bytes
            bPropertyName: [                    // "DeviceInterfaceGUID"
                b'D', 0, b'e', 0, b'v', 0, b'i', 0, b'c', 0, b'e', 0, b'I', 0, b'n', 0,
                b't', 0, b'e', 0, b'r', 0, b'f', 0, b'a', 0, b'c', 0, b'e', 0, b'G', 0,
                b'U', 0, b'I', 0, b'D', 0, 0, 0,
            ],
            dwPropertyDataLength: 0x0000_004E,  // 78 bytes
            bPropertyData: [                    // "{88BAE032-5A81-49f0-BC3D-A4FF138216D6}" (from winusb.inf)
                b'{', 0, b'8', 0, b'8', 0, b'b', 0, b'a', 0, b'e', 0, b'0', 0, b'3', 0,
                b'2', 0, b'-', 0, b'5', 0, b'a', 0, b'8', 0, b'1', 0, b'-', 0, b'4', 0,
                b'9', 0, b'f', 0, b'0', 0, b'-', 0, b'b', 0, b'c', 0, b'3', 0, b'd', 0,
                b'-', 0, b'a', 0, b'4', 0, b'f', 0, b'f', 0, b'1', 0, b'3', 0, b'8', 0,
                b'2', 0, b'1', 0, b'6', 0, b'd', 0, b'6', 0, b'}', 0, 0, 0
            ],
        }
    }
}

impl AsByteSliceIterator for ExtendedPropertiesFeatureDescriptor {}
