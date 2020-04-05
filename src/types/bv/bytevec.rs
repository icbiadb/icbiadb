use serde::{Serialize, Deserialize};

use crate::prelude::{BvContains, BvStartsWith, BvEndsWith};
use crate::utils::{
	serialize_to_bytevec,
	deserialize_bytevec,
};
use crate::slice::*;

use super::BvString;

use std::convert::TryFrom;




#[derive(Default, Serialize, Deserialize, Clone, Hash, Debug, Eq)]
pub struct ByteVec(Vec<u8>);

impl ByteVec {
	pub fn new() -> Self { ByteVec(Vec::new()) }
	pub fn from_obj<T: Sized + serde::ser::Serialize>(o: T) -> Self { serialize_to_bytevec(&o) }
	pub fn from_vec(v: Vec<u8>) -> Self { ByteVec(v) }
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

	pub fn extract<T: ?Sized + serde::de::DeserializeOwned>(&self) -> T {
		deserialize_bytevec(&self)
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

impl std::convert::From<&ByteVec> for ByteVec {
	fn from(other: &ByteVec) -> Self {
		ByteVec(other.to_vec())
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

impl std::cmp::PartialEq<&ByteVec> for ByteVec {
	fn eq(&self, other: &&ByteVec) -> bool {
		&self.0 == other.inner()
	}
}

impl std::cmp::PartialEq<BvString> for ByteVec {
	fn eq(&self, other: &BvString) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl std::cmp::PartialEq<&BvString> for ByteVec {
	fn eq(&self, other: &&BvString) -> bool {
		self.as_slice() == other.as_slice()
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
		self.0.as_slice() == other
	}
}

impl std::cmp::PartialEq<&[u8]> for ByteVec {
	fn eq(&self, other: &&[u8]) -> bool {
		&self.0.as_slice() == other
	}
}




impl PartialEq<i16> for &ByteVec {
	fn eq(&self, other: &i16) -> bool {
		self.as_i16() == *other
	}
}

impl PartialOrd<i16> for &ByteVec {
	fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
		let value = self.as_i16();
		return if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<i32> for &ByteVec {
	fn eq(&self, other: &i32) -> bool {
		self.as_i32() == *other
	}
}

impl PartialOrd<i32> for &ByteVec {
	fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
		let value = self.as_i32();
		return if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<i64> for &ByteVec {
	fn eq(&self, other: &i64) -> bool {
		self.as_i64() == *other
	}
}

impl PartialOrd<i64> for &ByteVec {
	fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
		let value = self.as_i64();
		return if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<i128> for &ByteVec {
	fn eq(&self, other: &i128) -> bool {
		self.as_i128() == *other
	}
}

impl PartialOrd<i128> for &ByteVec {
	fn partial_cmp(&self, other: &i128) -> Option<std::cmp::Ordering> {
		let value = self.as_i128();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u16> for &ByteVec {
	fn eq(&self, other: &u16) -> bool {
		self.as_u16() == *other
	}
}

impl PartialOrd<u16> for &ByteVec {
	fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
		let value = self.as_u16();
		return if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u32> for &ByteVec {
	fn eq(&self, other: &u32) -> bool {
		self.as_u32() == *other
	}
}

impl PartialOrd<u32> for &ByteVec {
	fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
		let value = self.as_u32();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u64> for &ByteVec {
	fn eq(&self, other: &u64) -> bool {
		self.as_u64() == *other
	}
}

impl PartialOrd<u64> for &ByteVec {
	fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
		let value = self.as_u64();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<usize> for &ByteVec {
	fn eq(&self, other: &usize) -> bool {
		self.as_usize() == *other
	}
}

impl PartialOrd<usize> for &ByteVec {
	fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
		let value = self.as_usize();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u128> for &ByteVec {
	fn eq(&self, other: &u128) -> bool {
		self.as_u128() == *other
	}
}

impl PartialOrd<u128> for &ByteVec {
	fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
		let value = self.as_u128();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<f32> for &ByteVec {
	fn eq(&self, other: &f32) -> bool {
		self.as_f32() == *other
	}
}

impl PartialOrd<f32> for &ByteVec {
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

impl PartialEq<f64> for &ByteVec {
	fn eq(&self, other: &f64) -> bool {
		self.as_f64() == *other
	}
}

impl PartialOrd<f64> for &ByteVec {
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


