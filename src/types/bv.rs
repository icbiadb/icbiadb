use serde::{Serialize, Deserialize};

use crate::prelude::{BvContains, BvStartsWith, BvEndsWith};
use crate::slice::*;


use std::convert::TryFrom;




#[derive(Serialize, Deserialize, Clone, Hash, Debug, Eq)]
pub struct ByteVec(Vec<u8>);

impl ByteVec {
	pub fn new() -> Self { ByteVec(Vec::new()) }
	pub fn from(v: Vec<u8>) -> Self { ByteVec(v) }
	pub fn inner(&self) -> &Vec<u8> { &self.0 }

	pub fn to_vec(&self) -> Vec<u8> {
		self.0.to_vec()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> std::slice::Iter<u8> {
		self.0.iter()
	}

	pub fn as_str(&self) -> &str {
		std::str::from_utf8(self.0.as_slice()).unwrap()
	}

	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
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


impl BvStartsWith<&str> for ByteVec {
	fn starts_with(&self, other: &str) -> bool {
		self[8..].starts_with(other.as_bytes())
	}
}

impl BvStartsWith<&[u8]> for ByteVec {
	fn starts_with(&self, other: &[u8]) -> bool {
		self[8..].starts_with(other)
	}
}

impl BvContains<&str> for ByteVec {
	fn contains(&self, other: &str) -> bool {
		contains_sequence(&self[8..], other.as_bytes())
	}
}

impl BvContains<&[u8]> for ByteVec {
	fn contains(&self, other: &[u8]) -> bool {
		contains_sequence(&self[8..], other)
	}
}

impl BvEndsWith<&str> for ByteVec {
	fn ends_with(&self, other: &str) -> bool {
		self.as_slice().ends_with(other.as_bytes())
	}
}

impl BvEndsWith<&[u8]> for ByteVec {
	fn ends_with(&self, other: &[u8]) -> bool {
		self.as_slice().ends_with(other)
	}
}


impl std::convert::From<&[u8]> for ByteVec {
	fn from(other: &[u8]) -> Self {
		ByteVec(other.to_vec())
	}
}

impl std::convert::From<Vec<u8>> for ByteVec {
	fn from(other: Vec<u8>) -> Self {
		ByteVec(other)
	}
}


impl std::ops::Index<usize> for ByteVec {
	type Output = u8;

	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}

impl std::ops::Index<std::ops::Range<usize>> for ByteVec {
	type Output = [u8];

	fn index(&self, i: std::ops::Range<usize>) -> &Self::Output {
		&self.0[..][i]
	}
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for ByteVec {
	type Output = [u8];

	fn index(&self, i: std::ops::RangeFrom<usize>) -> &Self::Output {
		&self.0[..][i]
	}
}

impl std::ops::Index<std::ops::RangeTo<usize>> for ByteVec {
	type Output = [u8];

	fn index(&self, i: std::ops::RangeTo<usize>) -> &Self::Output {
		&self.0[..][i]
	}
}


impl std::cmp::PartialEq<ByteVec> for ByteVec {
	fn eq(&self, other: &ByteVec) -> bool {
		&self.0 == other.inner()
	}
}

impl std::cmp::PartialEq<str> for ByteVec {
	fn eq(&self, other: &str) -> bool {
		if self.len() > 8 {
			return &self[8..] == other.as_bytes()
		}

		false
	}
}

impl std::cmp::PartialEq<&str> for ByteVec {
	fn eq(&self, other: &&str) -> bool {
		if self.len() > 8 {
			return &self[8..] == other.as_bytes()
		}

		false
	}
}

impl std::cmp::PartialEq<[u8]> for ByteVec {
	fn eq(&self, other: &[u8]) -> bool {
		if self.len() > 8 {
			return &self[8..] == other
		}

		false
	}
}

impl std::cmp::PartialEq<&[u8]> for ByteVec {
	fn eq(&self, other: &&[u8]) -> bool {
		if self.len() > 8 {
			return &&self[8..] == other
		}

		false
	}
}



#[derive(Serialize, Deserialize, Clone, Hash, Debug, Eq)]
pub struct BvString(Vec<u8>);

impl BvString {
	pub fn new() -> Self { BvString(Vec::new()) }
	pub fn from(v: Vec<u8>) -> Self { BvString(v) }
	pub fn inner(&self) -> &Vec<u8> { &self.0 }

	pub fn as_str(&self) -> &str {
		std::str::from_utf8(self.0.as_slice()).unwrap()
	}

	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
	}
	
	pub fn to_vec(&self) -> Vec<u8> {
		self.0.to_vec()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> std::slice::Iter<u8> {
		self.0.iter()
	}
}

impl std::fmt::Display for BvString {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", std::str::from_utf8(self.0.as_slice()).unwrap())
	}
}

impl BvContains<&str> for BvString {
	fn contains(&self, other: &str) -> bool {
		contains_sequence(self.as_slice(), other.as_bytes())
	}
}

impl BvContains<&[u8]> for BvString {
	fn contains(&self, other: &[u8]) -> bool {
		contains_sequence(self.as_slice(), other)
	}
}

impl BvStartsWith<&str> for BvString {
	fn starts_with(&self, other: &str) -> bool {
		self.as_slice().starts_with(other.as_bytes())
	}
}

impl BvStartsWith<&[u8]> for BvString {
	fn starts_with(&self, other: &[u8]) -> bool {
		self.as_slice().starts_with(other)
	}
}

impl BvEndsWith<&str> for BvString {
	fn ends_with(&self, other: &str) -> bool {
		self.as_slice().ends_with(other.as_bytes())
	}
}

impl BvEndsWith<&[u8]> for BvString {
	fn ends_with(&self, other: &[u8]) -> bool {
		self.as_slice().ends_with(other)
	}
}


impl std::convert::From<&[u8]> for BvString {
	fn from(other: &[u8]) -> Self {
		BvString(other.to_vec())
	}
}

impl std::convert::From<Vec<u8>> for BvString {
	fn from(other: Vec<u8>) -> Self {
		BvString(other)
	}
}


impl std::ops::Index<usize> for BvString {
	type Output = u8;

	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}

impl std::ops::Index<std::ops::Range<usize>> for BvString {
	type Output = [u8];

	fn index(&self, i: std::ops::Range<usize>) -> &Self::Output {
		&self.0[..][i]
	}
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for BvString {
	type Output = [u8];

	fn index(&self, i: std::ops::RangeFrom<usize>) -> &Self::Output {
		&self.0[..][i]
	}
}

impl std::ops::Index<std::ops::RangeTo<usize>> for BvString {
	type Output = [u8];

	fn index(&self, i: std::ops::RangeTo<usize>) -> &Self::Output {
		&self.0[..][i]
	}
}



impl std::cmp::PartialEq<BvString> for BvString {
	fn eq(&self, other: &BvString) -> bool {
		&self.0 == other.inner()
	}
}

impl std::cmp::PartialEq<String> for BvString {
	fn eq(&self, other: &String) -> bool {
		self.as_slice() == other.as_bytes()
	}
}

impl std::cmp::PartialEq<&String> for BvString {
	fn eq(&self, other: &&String) -> bool {
		self.as_slice() == other.as_bytes()
	}
}

impl std::cmp::PartialEq<str> for BvString {
	fn eq(&self, other: &str) -> bool {
		self.as_slice() == other.as_bytes()
	}
}

impl std::cmp::PartialEq<&str> for BvString {
	fn eq(&self, other: &&str) -> bool {
		self.as_slice() == other.as_bytes()
	}
}

impl std::cmp::PartialEq<[u8]> for BvString {
	fn eq(&self, other: &[u8]) -> bool {
		self.as_slice() == other
	}
}

impl std::cmp::PartialEq<&[u8]> for BvString {
	fn eq(&self, other: &&[u8]) -> bool {
		&self.as_slice() == other
	}
}

impl std::cmp::PartialEq<String> for &BvString {
	fn eq(&self, other: &String) -> bool {
		self.as_slice() == other.as_bytes()
	}
}
