use crate::prelude::{BvContains, BvEndsWith, BvStartsWith};
use crate::slice::*;

use super::*;

use std::convert::TryFrom;

#[derive(Default, Copy, Clone, Hash, Debug, Eq)]
pub struct ByteSlice<'a>(&'a [u8]);

impl<'a> ByteSlice<'a> {
    pub fn new(bs: &'a [u8]) -> Self {
        ByteSlice(bs)
    }
    //pub fn from_obj<T: Sized + serde::ser::Serialize>(o: T) -> Self { serialize_to_bytevec(&o) }
    pub fn inner(&self) -> &'a [u8] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.0.iter()
    }

    pub fn extract<T: ?Sized + serde::de::DeserializeOwned>(&self) {
        //deserialize_bytevec(&self)
        //deserialize_byteslice(&self)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.0).unwrap()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        usize::from_le_bytes(<[u8; 8]>::try_from(&self[..8]).unwrap())
    }

    pub fn as_u16(&self) -> u16 {
        u16::from_le_bytes(<[u8; 2]>::try_from(&self[..2]).unwrap())
    }

    pub fn as_u32(&self) -> u32 {
        u32::from_le_bytes(<[u8; 4]>::try_from(&self[..4]).unwrap())
    }

    pub fn as_u64(&self) -> u64 {
        u64::from_le_bytes(<[u8; 8]>::try_from(&self[..8]).unwrap())
    }

    pub fn as_u128(&self) -> u128 {
        u128::from_le_bytes(<[u8; 16]>::try_from(&self[..16]).unwrap())
    }

    pub fn as_i16(&self) -> i16 {
        i16::from_le_bytes(<[u8; 2]>::try_from(&self[..2]).unwrap())
    }

    pub fn as_i32(&self) -> i32 {
        i32::from_le_bytes(<[u8; 4]>::try_from(&self[..4]).unwrap())
    }

    pub fn as_i64(&self) -> i64 {
        i64::from_le_bytes(<[u8; 8]>::try_from(&self[..8]).unwrap())
    }

    pub fn as_i128(&self) -> i128 {
        i128::from_le_bytes(<[u8; 16]>::try_from(&self[..16]).unwrap())
    }

    pub fn as_f32(&self) -> f32 {
        f32::from_le_bytes(<[u8; 4]>::try_from(&self[..4]).unwrap())
    }

    pub fn as_f64(&self) -> f64 {
        f64::from_le_bytes(<[u8; 8]>::try_from(&self[..8]).unwrap())
    }
}

impl BvStartsWith<&str> for ByteSlice<'_> {
    fn starts_with(&self, other: &str) -> bool {
        self[8..].starts_with(other.as_bytes())
    }
}

impl BvStartsWith<&[u8]> for ByteSlice<'_> {
    fn starts_with(&self, other: &[u8]) -> bool {
        self[8..].starts_with(other)
    }
}

impl BvContains<&str> for ByteSlice<'_> {
    fn contains(&self, other: &str) -> bool {
        contains_sequence(&self[8..], other.as_bytes())
    }
}

impl BvContains<&[u8]> for ByteSlice<'_> {
    fn contains(&self, other: &[u8]) -> bool {
        contains_sequence(&self[8..], other)
    }
}

impl BvEndsWith<&str> for ByteSlice<'_> {
    fn ends_with(&self, other: &str) -> bool {
        self.as_slice().ends_with(other.as_bytes())
    }
}

impl BvEndsWith<&[u8]> for ByteSlice<'_> {
    fn ends_with(&self, other: &[u8]) -> bool {
        self.as_slice().ends_with(other)
    }
}

impl<'a> std::convert::From<&'a [u8]> for ByteSlice<'a> {
    fn from(other: &'a [u8]) -> Self {
        ByteSlice(other)
    }
}

impl<'a> std::convert::From<&'a Vec<u8>> for ByteSlice<'a> {
    fn from(other: &'a Vec<u8>) -> Self {
        ByteSlice(other.as_slice())
    }
}

impl<'a> std::convert::From<&'a ByteVec> for ByteSlice<'a> {
    fn from(other: &'a ByteVec) -> Self {
        ByteSlice(other.as_slice())
    }
}

impl std::ops::Index<usize> for ByteSlice<'_> {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for ByteSlice<'_> {
    type Output = [u8];

    fn index(&self, i: std::ops::Range<usize>) -> &Self::Output {
        &self.0[..][i]
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for ByteSlice<'_> {
    type Output = [u8];

    fn index(&self, i: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.0[..][i]
    }
}

impl std::ops::Index<std::ops::RangeTo<usize>> for ByteSlice<'_> {
    type Output = [u8];

    fn index(&self, i: std::ops::RangeTo<usize>) -> &Self::Output {
        &self.0[..][i]
    }
}

impl<'a> std::cmp::PartialEq<ByteSlice<'a>> for ByteSlice<'_> {
    fn eq(&self, other: &ByteSlice<'a>) -> bool {
        self.0 == other.as_slice()
    }
}

impl<'a> std::cmp::PartialEq<&ByteSlice<'a>> for ByteSlice<'_> {
    fn eq(&self, other: &&ByteSlice<'a>) -> bool {
        self.0 == other.as_slice()
    }
}

impl std::cmp::PartialEq<ByteVec> for ByteSlice<'_> {
    fn eq(&self, other: &ByteVec) -> bool {
        self.0 == other.as_slice()
    }
}

impl std::cmp::PartialEq<&ByteVec> for ByteSlice<'_> {
    fn eq(&self, other: &&ByteVec) -> bool {
        self.0 == other.as_slice()
    }
}

impl std::cmp::PartialEq<BvString> for ByteSlice<'_> {
    fn eq(&self, other: &BvString) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl std::cmp::PartialEq<&BvString> for ByteSlice<'_> {
    fn eq(&self, other: &&BvString) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl std::cmp::PartialEq<BvStr<'_>> for ByteSlice<'_> {
    fn eq(&self, other: &BvStr<'_>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl std::cmp::PartialEq<&BvStr<'_>> for ByteSlice<'_> {
    fn eq(&self, other: &&BvStr<'_>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl std::cmp::PartialEq<str> for ByteSlice<'_> {
    fn eq(&self, other: &str) -> bool {
        if self.len() > 8 {
            return &self[8..] == other.as_bytes();
        }

        false
    }
}

impl std::cmp::PartialEq<&str> for ByteSlice<'_> {
    fn eq(&self, other: &&str) -> bool {
        if self.len() > 8 {
            return &self[8..] == other.as_bytes();
        }

        false
    }
}

impl std::cmp::PartialEq<[u8]> for ByteSlice<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl std::cmp::PartialEq<&[u8]> for ByteSlice<'_> {
    fn eq(&self, other: &&[u8]) -> bool {
        &self.0 == other
    }
}

impl PartialEq<i16> for &ByteSlice<'_> {
    fn eq(&self, other: &i16) -> bool {
        self.as_i16() == *other
    }
}

impl PartialOrd<i16> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
        Some(self.as_i16().cmp(other))
    }
}

impl PartialEq<i32> for &ByteSlice<'_> {
    fn eq(&self, other: &i32) -> bool {
        self.as_i32() == *other
    }
}

impl PartialOrd<i32> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        Some(self.as_i32().cmp(other))
    }
}

impl PartialEq<i64> for &ByteSlice<'_> {
    fn eq(&self, other: &i64) -> bool {
        self.as_i64() == *other
    }
}

impl PartialOrd<i64> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        Some(self.as_i64().cmp(other))
    }
}

impl PartialEq<i128> for &ByteSlice<'_> {
    fn eq(&self, other: &i128) -> bool {
        self.as_i128() == *other
    }
}

impl PartialOrd<i128> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &i128) -> Option<std::cmp::Ordering> {
        Some(self.as_i128().cmp(other))
    }
}

impl PartialEq<u16> for &ByteSlice<'_> {
    fn eq(&self, other: &u16) -> bool {
        self.as_u16() == *other
    }
}

impl PartialOrd<u16> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        Some(self.as_u16().cmp(other))
    }
}

impl PartialEq<u32> for &ByteSlice<'_> {
    fn eq(&self, other: &u32) -> bool {
        self.as_u32() == *other
    }
}

impl PartialOrd<u32> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        Some(self.as_u32().cmp(other))
    }
}

impl PartialEq<u64> for &ByteSlice<'_> {
    fn eq(&self, other: &u64) -> bool {
        self.as_u64() == *other
    }
}

impl PartialOrd<u64> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        Some(self.as_u64().cmp(other))
    }
}

impl PartialEq<usize> for &ByteSlice<'_> {
    fn eq(&self, other: &usize) -> bool {
        self.as_usize() == *other
    }
}

impl PartialOrd<usize> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        Some(self.as_usize().cmp(other))
    }
}

impl PartialEq<u128> for &ByteSlice<'_> {
    fn eq(&self, other: &u128) -> bool {
        self.as_u128() == *other
    }
}

impl PartialOrd<u128> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
        Some(self.as_u128().cmp(other))
    }
}

impl PartialEq<f32> for &ByteSlice<'_> {
    fn eq(&self, other: &f32) -> bool {
        self.as_f32() == *other
    }
}

impl PartialOrd<f32> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        let value = self.as_f32();
        if value > *other {
            Some(std::cmp::Ordering::Greater)
        } else if value < *other {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl PartialEq<f64> for &ByteSlice<'_> {
    fn eq(&self, other: &f64) -> bool {
        self.as_f64() == *other
    }
}

impl PartialOrd<f64> for &ByteSlice<'_> {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        let value = self.as_f64();
        if value > *other {
            Some(std::cmp::Ordering::Greater)
        } else if value < *other {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}
