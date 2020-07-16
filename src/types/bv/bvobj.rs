use crate::prelude::{BvContains, BvEndsWith, BvStartsWith};

use std::convert::TryFrom;

use super::{BvStr, ByteSlice};

#[derive(Default, Clone, Debug)]
pub struct BvObj<'a> {
    type_name: BvStr<'a>,
    raw: ByteSlice<'a>,
}

impl<'a> BvObj<'a> {
    pub fn new(type_name: &'a [u8], raw: &'a [u8]) -> Self {
        BvObj {
            type_name: BvStr::new(type_name),
            raw: ByteSlice::new(raw),
        }
    }

    pub fn from_tuple(t: (&'a [u8], &'a [u8])) -> Self {
        BvObj {
            type_name: BvStr::new(t.0),
            raw: ByteSlice::new(t.1),
        }
    }

    pub fn type_name(&self) -> &BvStr<'a> {
        &self.type_name
    }

    pub fn raw(&self) -> &ByteSlice<'a> {
        &self.raw
    }

    pub fn mut_raw(&mut self) -> &mut ByteSlice<'a> {
        &mut self.raw
    }

    pub fn extract<T: Sized + serde::de::DeserializeOwned>(&self) {
        //self.raw.extract()
    }

    pub fn is_str(&self) -> bool {
        self.type_name == "str"
    }

    pub fn is_int(&self) -> bool {
        match self.type_name.as_slice() {
            // i8-i128
            [105, 56] => true,
            [105, 49, 54] => true,
            [105, 51, 50] => true,
            [105, 54, 52] => true,
            [105, 49, 50, 56] => true,
            _ => false,
        }
    }

    pub fn is_uint(&self) -> bool {
        match self.type_name.as_slice() {
            // u8-u128
            [117, 56] => true,
            [117, 49, 54] => true,
            [117, 51, 50] => true,
            [117, 54, 52] => true,
            [117, 49, 50, 56] => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self.type_name.as_slice() {
            // f32-f64
            [102, 51, 50] => true,
            [102, 54, 52] => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.raw.as_slice()).unwrap()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.raw.as_slice()
    }

    pub fn as_usize(&self) -> usize {
        usize::from_le_bytes(<[u8; 8]>::try_from(&self.raw[..8]).unwrap())
    }

    pub fn as_u16(&self) -> u16 {
        u16::from_le_bytes(<[u8; 2]>::try_from(&self.raw[..2]).unwrap())
    }

    pub fn as_u32(&self) -> u32 {
        u32::from_le_bytes(<[u8; 4]>::try_from(&self.raw[..4]).unwrap())
    }

    pub fn as_u64(&self) -> u64 {
        u64::from_le_bytes(<[u8; 8]>::try_from(&self.raw[..8]).unwrap())
    }

    pub fn as_u128(&self) -> u128 {
        u128::from_le_bytes(<[u8; 16]>::try_from(&self.raw[..16]).unwrap())
    }

    pub fn as_i16(&self) -> i16 {
        i16::from_le_bytes(<[u8; 2]>::try_from(&self.raw[..2]).unwrap())
    }

    pub fn as_i32(&self) -> i32 {
        i32::from_le_bytes(<[u8; 4]>::try_from(&self.raw[..4]).unwrap())
    }

    pub fn as_i64(&self) -> i64 {
        i64::from_le_bytes(<[u8; 8]>::try_from(&self.raw[..8]).unwrap())
    }

    pub fn as_i128(&self) -> i128 {
        i128::from_le_bytes(<[u8; 16]>::try_from(&self.raw[..16]).unwrap())
    }

    pub fn as_f32(&self) -> f32 {
        f32::from_le_bytes(<[u8; 4]>::try_from(&self.raw[..4]).unwrap())
    }

    pub fn as_f64(&self) -> f64 {
        f64::from_le_bytes(<[u8; 8]>::try_from(&self.raw[..8]).unwrap())
    }
}

impl<'a> std::ops::Deref for BvObj<'a> {
    type Target = ByteSlice<'a>;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<'a> std::ops::DerefMut for BvObj<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

impl BvStartsWith<&String> for BvObj<'_> {
    fn starts_with(&self, other: &String) -> bool {
        self.is_str() && self.raw.starts_with(other.as_bytes())
    }
}

impl BvStartsWith<&[u8]> for BvObj<'_> {
    fn starts_with(&self, other: &[u8]) -> bool {
        self.is_str() && self.raw.starts_with(other)
    }
}

impl BvStartsWith<&str> for BvObj<'_> {
    fn starts_with(&self, other: &str) -> bool {
        self.is_str() && self.raw.starts_with(other)
    }
}

impl BvContains<&String> for BvObj<'_> {
    fn contains(&self, other: &String) -> bool {
        self.is_str() && self.raw.contains(other.as_bytes())
    }
}

impl BvContains<&[u8]> for BvObj<'_> {
    fn contains(&self, other: &[u8]) -> bool {
        self.is_str() && self.raw.contains(other)
    }
}

impl BvContains<&str> for BvObj<'_> {
    fn contains(&self, other: &str) -> bool {
        self.is_str() && self.raw.contains(other)
    }
}

impl BvEndsWith<&String> for BvObj<'_> {
    fn ends_with(&self, other: &String) -> bool {
        self.is_str() && self.raw.ends_with(other.as_bytes())
    }
}

impl BvEndsWith<&[u8]> for BvObj<'_> {
    fn ends_with(&self, other: &[u8]) -> bool {
        self.is_str() && self.raw.ends_with(other)
    }
}

impl BvEndsWith<&str> for BvObj<'_> {
    fn ends_with(&self, other: &str) -> bool {
        self.is_str() && self.raw.ends_with(other)
    }
}

impl std::fmt::Display for BvObj<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "BvObj {{ type_name: {}, value: {:?} }}",
            self.type_name.as_str(),
            self.raw
        )
    }
}

impl<'a> PartialEq<BvObj<'a>> for BvObj<'_> {
    fn eq(&self, other: &BvObj<'a>) -> bool {
        self.raw == other.as_slice()
    }
}

impl<'a> PartialEq<&BvObj<'a>> for BvObj<'_> {
    fn eq(&self, other: &&BvObj<'a>) -> bool {
        self.raw == other.as_slice()
    }
}

impl PartialEq<[u8]> for BvObj<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        if self.is_str() {
            &self.raw[8..] == other
        } else {
            self.raw == other
        }
    }
}

impl PartialEq<str> for BvObj<'_> {
    fn eq(&self, other: &str) -> bool {
        self.is_str() && &self.raw[8..] == other.as_bytes()
    }
}

impl PartialEq<String> for BvObj<'_> {
    fn eq(&self, other: &String) -> bool {
        self.is_str() && &self.raw[8..] == other.as_bytes()
    }
}

impl PartialEq<&String> for BvObj<'_> {
    fn eq(&self, other: &&String) -> bool {
        self.is_str() && &self.raw[8..] == other.as_bytes()
    }
}

// TODO
// Byte comparison and such instead

impl PartialEq<i16> for BvObj<'_> {
    fn eq(&self, other: &i16) -> bool {
        self.is_int() && self.raw.as_i16() == *other
    }
}

impl PartialOrd<i16> for BvObj<'_> {
    fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i16().cmp(other))
    }
}

impl PartialEq<i32> for BvObj<'_> {
    fn eq(&self, other: &i32) -> bool {
        self.is_int() && self.raw.as_i32() == *other
    }
}

impl PartialOrd<i32> for BvObj<'_> {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i32().cmp(other))
    }
}

impl PartialEq<i64> for BvObj<'_> {
    fn eq(&self, other: &i64) -> bool {
        self.is_int() && self.raw.as_i64() == *other
    }
}

impl PartialOrd<i64> for BvObj<'_> {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i64().cmp(other))
    }
}

impl PartialEq<i128> for BvObj<'_> {
    fn eq(&self, other: &i128) -> bool {
        self.is_uint() && self.raw.as_i128() == *other
    }
}

impl PartialOrd<i128> for BvObj<'_> {
    fn partial_cmp(&self, other: &i128) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_i128().cmp(other))
    }
}

impl PartialEq<u16> for BvObj<'_> {
    fn eq(&self, other: &u16) -> bool {
        self.is_uint() && self.raw.as_u16() == *other
    }
}

impl PartialOrd<u16> for BvObj<'_> {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u16().cmp(other))
    }
}

impl PartialEq<u32> for BvObj<'_> {
    fn eq(&self, other: &u32) -> bool {
        self.is_uint() && self.raw.as_u32() == *other
    }
}

impl PartialOrd<u32> for BvObj<'_> {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u32().cmp(other))
    }
}

impl PartialEq<u64> for BvObj<'_> {
    fn eq(&self, other: &u64) -> bool {
        self.is_uint() && self.raw.as_u64() == *other
    }
}

impl PartialOrd<u64> for BvObj<'_> {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u64().cmp(other))
    }
}

impl PartialEq<usize> for BvObj<'_> {
    fn eq(&self, other: &usize) -> bool {
        self.is_uint() && self.raw.as_usize() == *other
    }
}

impl PartialOrd<usize> for BvObj<'_> {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_usize().cmp(other))
    }
}

impl PartialEq<u128> for BvObj<'_> {
    fn eq(&self, other: &u128) -> bool {
        self.is_uint() && self.raw.as_u128() == *other
    }
}

impl PartialOrd<u128> for BvObj<'_> {
    fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u128().cmp(other))
    }
}

impl PartialEq<i16> for &BvObj<'_> {
    fn eq(&self, other: &i16) -> bool {
        self.is_int() && self.raw.as_i16() == *other
    }
}

impl PartialOrd<i16> for &BvObj<'_> {
    fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i16().cmp(other))
    }
}

impl PartialEq<i32> for &BvObj<'_> {
    fn eq(&self, other: &i32) -> bool {
        self.is_int() && self.raw.as_i32() == *other
    }
}

impl PartialOrd<i32> for &BvObj<'_> {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i32().cmp(other))
    }
}

impl PartialEq<i64> for &BvObj<'_> {
    fn eq(&self, other: &i64) -> bool {
        self.is_int() && self.raw.as_i64() == *other
    }
}

impl PartialOrd<i64> for &BvObj<'_> {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i64().cmp(other))
    }
}

impl PartialEq<i128> for &BvObj<'_> {
    fn eq(&self, other: &i128) -> bool {
        self.is_int() && self.raw.as_i128() == *other
    }
}

impl PartialOrd<i128> for &BvObj<'_> {
    fn partial_cmp(&self, other: &i128) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i128().cmp(other))
    }
}

impl PartialEq<u16> for &BvObj<'_> {
    fn eq(&self, other: &u16) -> bool {
        self.is_uint() && self.raw.as_u16() == *other
    }
}

impl PartialOrd<u16> for &BvObj<'_> {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u16().cmp(other))
    }
}

impl PartialEq<u32> for &BvObj<'_> {
    fn eq(&self, other: &u32) -> bool {
        self.is_uint() && self.raw.as_u32() == *other
    }
}

impl PartialOrd<u32> for &BvObj<'_> {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u32().cmp(other))
    }
}

impl PartialEq<u64> for &BvObj<'_> {
    fn eq(&self, other: &u64) -> bool {
        self.is_uint() && self.raw.as_u64() == *other
    }
}

impl PartialOrd<u64> for &BvObj<'_> {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u64().cmp(other))
    }
}

impl PartialEq<u128> for &BvObj<'_> {
    fn eq(&self, other: &u128) -> bool {
        self.is_uint() && self.raw.as_u128() == *other
    }
}

impl PartialOrd<u128> for &BvObj<'_> {
    fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u128().cmp(other))
    }
}

impl PartialEq<usize> for &BvObj<'_> {
    fn eq(&self, other: &usize) -> bool {
        self.is_uint() && self.raw.as_usize() == *other
    }
}

impl PartialOrd<usize> for &BvObj<'_> {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_usize().cmp(other))
    }
}

impl PartialEq<f32> for &BvObj<'_> {
    fn eq(&self, other: &f32) -> bool {
        self.is_float() && self.raw.as_f32() == *other
    }
}

impl PartialOrd<f32> for &BvObj<'_> {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        if !self.is_float() {
            return None;
        }

        let value = self.raw.as_f32();
        if value > *other {
            Some(std::cmp::Ordering::Greater)
        } else if value < *other {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl PartialEq<f64> for &BvObj<'_> {
    fn eq(&self, other: &f64) -> bool {
        self.is_float() && self.raw.as_f64() == *other
    }
}

impl PartialOrd<f64> for &BvObj<'_> {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        if !self.is_float() {
            return None;
        }

        let value = self.raw.as_f64();
        if value > *other {
            Some(std::cmp::Ordering::Greater)
        } else if value < *other {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}
