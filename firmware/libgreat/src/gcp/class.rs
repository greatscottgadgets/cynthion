//! Great Communications Protocol Class Registry

use zerocopy::byteorder::{LittleEndian, U32};

// - Classes ------------------------------------------------------------------

/// Collection of Great Communications Protocol classes
#[derive(Copy, Clone)]
pub struct Classes(pub &'static [Class]);

impl Classes {
    #[must_use]
    pub fn class(&self, id: ClassId) -> Option<&Class> {
        self.0.iter().find(|&class| class.id == id)
    }

    #[must_use]
    pub fn new() -> Self {
        Self(&[])
    }
}

impl Default for Classes {
    fn default() -> Self {
        Self::new()
    }
}

impl core::ops::Deref for Classes {
    type Target = &'static [Class];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// - Class --------------------------------------------------------------------

/// Great Communications Protocol class
#[derive(Copy, Clone)]
pub struct Class {
    pub id: ClassId,
    pub name: &'static str,
    pub docs: &'static str,
    pub verbs: &'static [Verb],
}

impl Class {
    #[must_use]
    pub fn verb(&self, id: u32) -> Option<&Verb> {
        self.verbs.iter().find(|&verb| verb.id == id)
    }
}

// - Verb ---------------------------------------------------------------------

/// Great Communications Protocol verb
#[derive(Copy, Clone)]
pub struct Verb {
    pub id: u32,
    pub name: &'static str,
    pub in_signature: &'static str,
    pub in_param_names: &'static str,
    pub out_signature: &'static str,
    pub out_param_names: &'static str,
    pub doc: &'static str,
}

/// Great Communications Protocol verb descriptor
#[repr(u8)]
pub enum VerbDescriptor {
    OutSignature = 0,
    InSignature = 1,
    Doc = 2,
    OutParamNames = 3,
    InParamNames = 4,
    Unknown(u8),
}

impl core::convert::From<u8> for VerbDescriptor {
    fn from(value: u8) -> Self {
        use VerbDescriptor::*;
        match value {
            0 => OutSignature,
            1 => InSignature,
            2 => Doc,
            3 => OutParamNames,
            4 => InParamNames,
            _ => Unknown(value),
        }
    }
}

// - ClassId ------------------------------------------------------------------

/// Great Communications Protocol class id
#[repr(u32)]
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum ClassId {
    core = 0x0000,
    firmware = 0x0001,
    selftest = 0x0011,
    gpio = 0x0103,
    greatdancer = 0x0104,
    moondancer = 0x0120,
    unsupported(u32),
}

impl core::convert::From<u32> for ClassId {
    fn from(value: u32) -> Self {
        match value {
            0x0000 => ClassId::core,
            0x0001 => ClassId::firmware,
            0x0011 => ClassId::selftest,
            0x0103 => ClassId::gpio,
            0x0104 => ClassId::greatdancer,
            0x0120 => ClassId::moondancer,
            _ => ClassId::unsupported(value),
        }
    }
}

impl ClassId {
    #[must_use]
    pub const fn into_u32(&self) -> u32 {
        match self {
            ClassId::core => 0x0000,
            ClassId::firmware => 0x0001,
            ClassId::selftest => 0x0011,
            ClassId::gpio => 0x0103,
            ClassId::greatdancer => 0x0104,
            ClassId::moondancer => 0x0120,
            ClassId::unsupported(value) => *value,
        }
    }
}

impl core::convert::From<U32<LittleEndian>> for ClassId {
    fn from(value: U32<LittleEndian>) -> Self {
        ClassId::from(value.get())
    }
}
