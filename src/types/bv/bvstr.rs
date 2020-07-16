use super::{BvObject, BvString};
use crate::prelude::{BvContains, BvEndsWith, BvStartsWith};
use crate::slice::*;

#[derive(Default, Debug, Clone, Eq)]
pub struct BvStr<'a>(&'a [u8]);

impl<'a> BvStr<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        BvStr(s)
    }

    pub fn from_bvobject(o: &'a BvObject) -> Self {
        BvStr::new(&o.as_slice()[8..])
    }

    pub fn inner(&self) -> &[u8] {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.0).unwrap()
    }

    pub fn as_slice(&self) -> &'a [u8] {
        self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_bvstring(&self) -> BvString {
        BvString::from(self.0.to_vec())
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
}

impl std::fmt::Display for BvStr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.0).unwrap())
    }
}

impl BvContains<&str> for BvStr<'_> {
    fn contains(&self, other: &str) -> bool {
        contains_sequence(self.as_slice(), other.as_bytes())
    }
}

impl BvContains<&[u8]> for BvStr<'_> {
    fn contains(&self, other: &[u8]) -> bool {
        contains_sequence(self.as_slice(), other)
    }
}

impl BvStartsWith<&str> for BvStr<'_> {
    fn starts_with(&self, other: &str) -> bool {
        self.as_slice().starts_with(other.as_bytes())
    }
}

impl BvStartsWith<&[u8]> for BvStr<'_> {
    fn starts_with(&self, other: &[u8]) -> bool {
        self.as_slice().starts_with(other)
    }
}

impl BvEndsWith<&str> for BvStr<'_> {
    fn ends_with(&self, other: &str) -> bool {
        self.as_slice().ends_with(other.as_bytes())
    }
}

impl BvEndsWith<&[u8]> for BvStr<'_> {
    fn ends_with(&self, other: &[u8]) -> bool {
        self.as_slice().ends_with(other)
    }
}

impl<'a> std::convert::From<&'a [u8]> for BvStr<'a> {
    fn from(other: &'a [u8]) -> Self {
        BvStr(other)
    }
}

impl<'a> std::convert::From<&'a Vec<u8>> for BvStr<'a> {
    fn from(other: &'a Vec<u8>) -> Self {
        BvStr(other)
    }
}

impl<'a> std::convert::From<&'a String> for BvStr<'a> {
    fn from(other: &'a String) -> Self {
        BvStr(other.as_bytes())
    }
}

impl<'a> std::convert::From<&'a str> for BvStr<'a> {
    fn from(other: &'a str) -> Self {
        BvStr(other.as_bytes())
    }
}

impl std::ops::Index<usize> for BvStr<'_> {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for BvStr<'_> {
    type Output = [u8];

    fn index(&self, i: std::ops::Range<usize>) -> &Self::Output {
        &self.0[..][i]
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for BvStr<'_> {
    type Output = [u8];

    fn index(&self, i: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.0[..][i]
    }
}

impl std::ops::Index<std::ops::RangeTo<usize>> for BvStr<'_> {
    type Output = [u8];

    fn index(&self, i: std::ops::RangeTo<usize>) -> &Self::Output {
        &self.0[..][i]
    }
}

impl std::cmp::PartialEq<&BvString> for BvStr<'_> {
    fn eq(&self, other: &&BvString) -> bool {
        self.0 == other.as_slice()
    }
}

impl std::cmp::PartialEq<BvStr<'_>> for BvStr<'_> {
    fn eq(&self, other: &BvStr<'_>) -> bool {
        self.0 == other.inner()
    }
}

impl std::cmp::PartialEq<String> for BvStr<'_> {
    fn eq(&self, other: &String) -> bool {
        self.as_slice() == other.as_bytes()
    }
}

impl std::cmp::PartialEq<&String> for BvStr<'_> {
    fn eq(&self, other: &&String) -> bool {
        self.as_slice() == other.as_bytes()
    }
}

impl std::cmp::PartialEq<str> for BvStr<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_slice() == other.as_bytes()
    }
}

impl std::cmp::PartialEq<&str> for BvStr<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.as_slice() == other.as_bytes()
    }
}

impl std::cmp::PartialEq<[u8]> for BvStr<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        self.as_slice() == other
    }
}

impl std::cmp::PartialEq<&[u8]> for BvStr<'_> {
    fn eq(&self, other: &&[u8]) -> bool {
        &self.as_slice() == other
    }
}

impl std::cmp::PartialEq<String> for &BvStr<'_> {
    fn eq(&self, other: &String) -> bool {
        self.as_slice() == other.as_bytes()
    }
}
