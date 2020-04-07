use serde::{Serialize, Deserialize};

use crate::prelude::{BvContains, BvStartsWith, BvEndsWith};
use crate::slice::*;
use crate::types::BvStr;



#[derive(Default, Serialize, Deserialize, Debug, Clone, Hash, Eq)]
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

impl std::convert::From<&BvString> for BvString {
	fn from(other: &BvString) -> Self {
		BvString(other.to_vec())
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

impl std::convert::From<String> for BvString {
	fn from(other: String) -> Self {
		BvString(other.as_bytes().to_vec())
	}
}

impl std::convert::From<&String> for BvString {
	fn from(other: &String) -> Self {
		BvString(other.as_bytes().to_vec())
	}
}

impl std::convert::From<&str> for BvString {
	fn from(other: &str) -> Self {
		BvString(other.as_bytes().to_vec())
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

impl std::cmp::PartialEq<&BvString> for BvString {
	fn eq(&self, other: &&BvString) -> bool {
		&self.0 == other.inner()
	}
}

impl std::cmp::PartialEq<BvStr<'_>> for BvString {
	fn eq(&self, other: &BvStr<'_>) -> bool {
		self.0.as_slice() == other.inner()
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


