use serde::{Deserialize, Serialize};

use crate::prelude::{BvContains, BvEndsWith, BvStartsWith};
use crate::utils::{normalize_type_name, serialize_to_bytevec};

use super::{BvString, ByteVec};

/// Wrapper for serialized objects by bincode
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct BvObject {
    type_name: BvString,
    raw: ByteVec,
}

impl BvObject {
    pub fn from<T: Sized + serde::ser::Serialize>(o: T) -> Self {
        BvObject {
            type_name: normalize_type_name(std::any::type_name::<T>().as_bytes()).into(),
            raw: serialize_to_bytevec(&o),
        }
    }

    pub fn from_raw(t: Vec<u8>, v: Vec<u8>) -> Self {
        BvObject {
            type_name: t.into(),
            raw: v.into(),
        }
    }

    pub fn from_tuple(t: (&[u8], &[u8])) -> Self {
        BvObject {
            type_name: t.0.into(),
            raw: t.1.into(),
        }
    }

    pub fn type_name(&self) -> &BvString {
        &self.type_name
    }

    pub fn raw(&self) -> &ByteVec {
        &self.raw
    }

    pub fn mut_raw(&mut self) -> &mut ByteVec {
        &mut self.raw
    }

    pub fn extract<T: Sized + serde::de::DeserializeOwned>(&self) -> T {
        self.raw.extract()
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
        std::str::from_utf8(&self.raw.as_slice()[8..]).unwrap()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.raw.as_slice()
    }

    pub fn as_str_slice(&self) -> &[u8] {
        &self.raw.as_slice()[8..]
    }
}

impl std::ops::Deref for BvObject {
    type Target = ByteVec;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl std::ops::DerefMut for BvObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

impl std::convert::From<(&[u8], &[u8])> for BvObject {
    fn from(other: (&[u8], &[u8])) -> Self {
        BvObject::from_tuple(other)
    }
}

impl std::convert::From<&BvObject> for BvObject {
    fn from(other: &BvObject) -> Self {
        BvObject {
            type_name: other.type_name().into(),
            raw: other.raw().into(),
        }
    }
}

impl BvStartsWith<&String> for BvObject {
    fn starts_with(&self, other: &String) -> bool {
        self.is_str() && self.raw.starts_with(other.as_bytes())
    }
}

impl BvStartsWith<&[u8]> for BvObject {
    fn starts_with(&self, other: &[u8]) -> bool {
        self.is_str() && self.raw.starts_with(other)
    }
}

impl BvStartsWith<&str> for BvObject {
    fn starts_with(&self, other: &str) -> bool {
        self.is_str() && self.raw.starts_with(other)
    }
}

impl BvContains<&String> for BvObject {
    fn contains(&self, other: &String) -> bool {
        self.is_str() && self.raw.contains(other.as_bytes())
    }
}

impl BvContains<&[u8]> for BvObject {
    fn contains(&self, other: &[u8]) -> bool {
        self.is_str() && self.raw.contains(other)
    }
}

impl BvContains<&str> for BvObject {
    fn contains(&self, other: &str) -> bool {
        self.is_str() && self.raw.contains(other)
    }
}

impl BvEndsWith<&String> for BvObject {
    fn ends_with(&self, other: &String) -> bool {
        self.is_str() && self.raw.ends_with(other.as_bytes())
    }
}

impl BvEndsWith<&[u8]> for BvObject {
    fn ends_with(&self, other: &[u8]) -> bool {
        self.is_str() && self.raw.ends_with(other)
    }
}

impl BvEndsWith<&str> for BvObject {
    fn ends_with(&self, other: &str) -> bool {
        self.is_str() && self.raw.ends_with(other)
    }
}

impl std::fmt::Display for BvObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "BvObject {{ type_name: {}, value: {:?} }}",
            self.type_name.as_str(),
            self.raw
        )
    }
}

impl PartialEq<BvObject> for BvObject {
    fn eq(&self, other: &BvObject) -> bool {
        self.raw == other.as_slice()
    }
}

impl PartialEq<&BvObject> for BvObject {
    fn eq(&self, other: &&BvObject) -> bool {
        self.raw == other.as_slice()
    }
}

impl PartialEq<bool> for &BvObject {
    fn eq(&self, other: &bool) -> bool {
        if *other {
            self.raw[0] == 1
        } else {
            self.raw[0] == 0
        }
    }
}

impl PartialEq<[u8]> for BvObject {
    fn eq(&self, other: &[u8]) -> bool {
        if self.is_str() {
            &self.raw[8..] == other
        } else {
            self.raw == other
        }
    }
}

impl PartialEq<str> for BvObject {
    fn eq(&self, other: &str) -> bool {
        self.is_str() && &self.raw[8..] == other.as_bytes()
    }
}

impl PartialEq<String> for BvObject {
    fn eq(&self, other: &String) -> bool {
        self.is_str() && &self.raw[8..] == other.as_bytes()
    }
}

impl PartialEq<&String> for BvObject {
    fn eq(&self, other: &&String) -> bool {
        self.is_str() && &self.raw[8..] == other.as_bytes()
    }
}

impl PartialEq<i16> for BvObject {
    fn eq(&self, other: &i16) -> bool {
        self.is_int() && self.raw.as_i16() == *other
    }
}

impl PartialOrd<i16> for BvObject {
    fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i16().cmp(other))
    }
}

impl PartialEq<i32> for BvObject {
    fn eq(&self, other: &i32) -> bool {
        self.is_int() && self.raw.as_i32() == *other
    }
}

impl PartialOrd<i32> for BvObject {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i32().cmp(other))
    }
}

impl PartialEq<i64> for BvObject {
    fn eq(&self, other: &i64) -> bool {
        self.is_int() && self.raw.as_i64() == *other
    }
}

impl PartialOrd<i64> for BvObject {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i64().cmp(other))
    }
}

impl PartialEq<i128> for BvObject {
    fn eq(&self, other: &i128) -> bool {
        self.is_uint() && self.raw.as_i128() == *other
    }
}

impl PartialOrd<i128> for BvObject {
    fn partial_cmp(&self, other: &i128) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_i128().cmp(other))
    }
}

impl PartialEq<u16> for BvObject {
    fn eq(&self, other: &u16) -> bool {
        self.is_uint() && self.raw.as_u16() == *other
    }
}

impl PartialOrd<u16> for BvObject {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u16().cmp(other))
    }
}

impl PartialEq<u32> for BvObject {
    fn eq(&self, other: &u32) -> bool {
        self.is_uint() && self.raw.as_u32() == *other
    }
}

impl PartialOrd<u32> for BvObject {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u32().cmp(other))
    }
}

impl PartialEq<u64> for BvObject {
    fn eq(&self, other: &u64) -> bool {
        self.is_uint() && self.raw.as_u64() == *other
    }
}

impl PartialOrd<u64> for BvObject {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u64().cmp(other))
    }
}

impl PartialEq<usize> for BvObject {
    fn eq(&self, other: &usize) -> bool {
        self.is_uint() && self.raw.as_usize() == *other
    }
}

impl PartialOrd<usize> for BvObject {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_usize().cmp(other))
    }
}

impl PartialEq<u128> for BvObject {
    fn eq(&self, other: &u128) -> bool {
        self.is_uint() && self.raw.as_u128() == *other
    }
}

impl PartialOrd<u128> for BvObject {
    fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u128().cmp(other))
    }
}

impl PartialEq<f32> for BvObject {
    fn eq(&self, other: &f32) -> bool {
        self.is_float() && self.raw.as_f32() == *other
    }
}

impl PartialOrd<f32> for BvObject {
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

impl PartialEq<f64> for BvObject {
    fn eq(&self, other: &f64) -> bool {
        self.is_float() && self.raw.as_f64() == *other
    }
}

impl PartialOrd<f64> for BvObject {
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

impl PartialEq<i8> for &BvObject {
    fn eq(&self, other: &i8) -> bool {
        self.is_int() && self.raw.as_i8() == *other
    }
}

impl PartialEq<i16> for &BvObject {
    fn eq(&self, other: &i16) -> bool {
        self.is_int() && self.raw.as_i16() == *other
    }
}

impl PartialOrd<i16> for &BvObject {
    fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i16().cmp(other))
    }
}

impl PartialEq<i32> for &BvObject {
    fn eq(&self, other: &i32) -> bool {
        self.is_int() && self.raw.as_i32() == *other
    }
}

impl PartialOrd<i32> for &BvObject {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i32().cmp(other))
    }
}

impl PartialEq<i64> for &BvObject {
    fn eq(&self, other: &i64) -> bool {
        self.is_int() && self.raw.as_i64() == *other
    }
}

impl PartialOrd<i64> for &BvObject {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i64().cmp(other))
    }
}

impl PartialEq<i128> for &BvObject {
    fn eq(&self, other: &i128) -> bool {
        self.is_int() && self.raw.as_i128() == *other
    }
}

impl PartialOrd<i128> for &BvObject {
    fn partial_cmp(&self, other: &i128) -> Option<std::cmp::Ordering> {
        if !self.is_int() {
            return None;
        }

        Some(self.raw.as_i128().cmp(other))
    }
}

impl PartialEq<u16> for &BvObject {
    fn eq(&self, other: &u16) -> bool {
        self.is_uint() && self.raw.as_u16() == *other
    }
}

impl PartialOrd<u16> for &BvObject {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u16().cmp(other))
    }
}

impl PartialEq<u32> for &BvObject {
    fn eq(&self, other: &u32) -> bool {
        self.is_uint() && self.raw.as_u32() == *other
    }
}

impl PartialOrd<u32> for &BvObject {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u32().cmp(other))
    }
}

impl PartialEq<u64> for &BvObject {
    fn eq(&self, other: &u64) -> bool {
        self.is_uint() && self.raw.as_u64() == *other
    }
}

impl PartialOrd<u64> for &BvObject {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u64().cmp(other))
    }
}

impl PartialEq<u128> for &BvObject {
    fn eq(&self, other: &u128) -> bool {
        self.is_uint() && self.raw.as_u128() == *other
    }
}

impl PartialOrd<u128> for &BvObject {
    fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_u128().cmp(other))
    }
}

impl PartialEq<usize> for &BvObject {
    fn eq(&self, other: &usize) -> bool {
        self.is_uint() && self.raw.as_usize() == *other
    }
}

impl PartialOrd<usize> for &BvObject {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        if !self.is_uint() {
            return None;
        }

        Some(self.raw.as_usize().cmp(other))
    }
}

impl PartialEq<f32> for &BvObject {
    fn eq(&self, other: &f32) -> bool {
        self.is_float() && self.raw.as_f32() == *other
    }
}

impl PartialOrd<f32> for &BvObject {
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

impl PartialEq<f64> for &BvObject {
    fn eq(&self, other: &f64) -> bool {
        self.is_float() && self.raw.as_f64() == *other
    }
}

impl PartialOrd<f64> for &BvObject {
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
