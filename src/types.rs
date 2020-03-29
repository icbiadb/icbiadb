use crate::{
	prelude::*,
	utils::*,
	slice,
};
use std::convert::TryFrom;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Hash, Debug)]
pub struct ByteVec(Vec<u8>);

impl ByteVec {
	pub fn new() -> Self { ByteVec(Vec::new()) }
	pub fn from(v: Vec<u8>) -> Self { ByteVec(v) }
	pub fn inner(&self) -> &Vec<u8> { &self.0 }
}

impl std::ops::Deref for ByteVec {
	type Target = Vec<u8>;

	fn deref(&self) -> &Self::Target {
		self.inner()
	}
}

impl std::cmp::Eq for ByteVec {}

impl std::cmp::PartialEq<ByteVec> for ByteVec {
	fn eq(&self, other: &ByteVec) -> bool {
		&self.0 == other.inner()
	}
}

impl std::cmp::PartialEq<str> for ByteVec {
	fn eq(&self, other: &str) -> bool {
		&self[8..] == other.as_bytes()
	}
}

impl std::cmp::PartialEq<&str> for ByteVec {
	fn eq(&self, other: &&str) -> bool {
		&self[8..] == other.as_bytes()
	}
}

impl std::cmp::PartialEq<[u8]> for ByteVec {
	fn eq(&self, other: &[u8]) -> bool {
		&self[8..] == other
	}
}

impl std::cmp::PartialEq<&[u8]> for ByteVec {
	fn eq(&self, other: &&[u8]) -> bool {
		&&self[8..] == other
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

#[derive(Debug)]
pub struct OwnedRecord {
	key: Vec<u8>,
	type_name: Vec<u8>,
	value: Vec<u8>,
}

impl OwnedRecord {
	pub fn new(key: Vec<u8>, type_name: Vec<u8>, value: Vec<u8>) -> Self {
		OwnedRecord {
			key: key,
			type_name: type_name,
			value: value
		}
	}

}

impl RecordWrite for OwnedRecord {
	fn set_value<T: ?Sized + serde::ser::Serialize>(&mut self, v: &T) {
		self.type_name = std::any::type_name::<T>().as_bytes().to_vec();
		self.value = serialize(v);
	}
}

impl RecordRead for OwnedRecord {
	fn key(&self) -> &str {
		std::str::from_utf8(&self.key).unwrap()
	}

	fn raw_key(&self) -> &[u8] {
		&self.key
	}

	fn type_name(&self) -> &str {	
		std::str::from_utf8(&self.type_name).unwrap()
	}

	fn raw_type_name(&self) -> &[u8] {
		&self.type_name
	}

	fn value<T: ?Sized + serde::de::DeserializeOwned>(&self) -> T {
		bincode::deserialize::<T>(&self.value).expect(&format!("Failed to deserialize value, {:?}", self))
	}

	fn raw_value(&self) -> &[u8] {
		&self.value
	}

	fn as_tuple(&self) -> (&[u8], &[u8], &[u8]) {
		(&self.key, &self.type_name, &self.value)
	}

	fn to_tuple(&self) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
		(self.key.to_vec(), self.type_name.to_vec(), self.value.to_vec())
	}
}


// TODO
// Cache deserialization here
pub enum TempValue<'a> {
	I32(i32),
	Raw(&'a [u8]),
	None,
}


pub struct Record<'a> {
	key: &'a [u8],
	type_name: &'a [u8],
	value: &'a [u8],
}

impl<'a> Record<'a> {
	pub fn new(key: &'a [u8], type_name: &'a [u8], value: &'a [u8]) -> Self {
		Record {
			key: key,
			type_name: type_name,
			value: value,
		}
	}

	pub fn is_int(&self) -> bool {
		let tn = self.raw_type_name();

		match tn {
			// u8-u64
			[117, 56] => true,
			[117, 49, 54] => true,
			[117, 51, 50] => true,
			[117, 54, 52] => true,
			// I8-i64
			[105, 56] => true,
			[105, 49, 54] => true,
			[105, 51, 50] => true,
			[105, 54, 52] => true,
			// Float 32-64
			[102, 51, 50] => true,
			[102, 54, 52] => true,
			_ => false
		}
	}
}

impl RecordRead for Record<'_> {
	fn key(&self) -> &str {
		std::str::from_utf8(&self.key).unwrap()
	}

	fn raw_key(&self) -> &[u8] {
		&self.key
	}

	fn type_name(&self) -> &str {	
		std::str::from_utf8(&self.type_name).unwrap()
	}

	fn raw_type_name(&self) -> &[u8] {
		self.type_name
	}

	fn value<T: ?Sized + serde::de::DeserializeOwned>(&self) -> T {
		bincode::deserialize::<T>(&self.value).expect(&format!("Failed to deserialize value, make sure it's the correct type: {}", self))
	}

	fn raw_value(&self) -> &[u8] {
		&self.value
	}

	fn as_tuple(&self) -> (&[u8], &[u8], &[u8]) {
		(self.key, self.type_name, self.value)
	}

	fn to_tuple(&self) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
		(self.key.to_vec(), self.type_name.to_vec(), self.value.to_vec())
	}
}



impl<'a> std::convert::From<&'a (Vec<u8>, Vec<u8>, Vec<u8>)> for Record<'a> {
	fn from(tuple: &'a (Vec<u8>, Vec<u8>, Vec<u8>)) -> Self {
		let (k, t, v) = tuple;
		Record::new(k, t, v)
	}
}

impl std::fmt::Display for Record<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Record {{ key: {:?}, type_name: {}, value: {:?} }}", self.key(), self.type_name(), self.value)
	}
}


// TODO
// Byte comparison and such instead

impl PartialEq<i16> for Record<'_> {
	fn eq(&self, other: &i16) -> bool {
		if self.is_int() {
			let r = i16::from_le_bytes(<[u8; 2]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<i16> for Record<'_> {
	fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = i16::from_le_bytes(<[u8; 2]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<i32> for Record<'_> {
	fn eq(&self, other: &i32) -> bool {
		if self.is_int() {
			let r = i32::from_le_bytes(<[u8; 4]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<i32> for Record<'_> {
	fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = i32::from_le_bytes(<[u8; 4]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<i64> for Record<'_> {
	fn eq(&self, other: &i64) -> bool {
		if self.is_int() {
			let r = i64::from_le_bytes(<[u8; 8]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<i64> for Record<'_> {
	fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = i64::from_le_bytes(<[u8; 8]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<u16> for Record<'_> {
	fn eq(&self, other: &u16) -> bool {
		if self.is_int() {
			let r = u16::from_le_bytes(<[u8; 2]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<u16> for Record<'_> {
	fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = u16::from_le_bytes(<[u8; 2]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<u32> for Record<'_> {
	fn eq(&self, other: &u32) -> bool {
		if self.is_int() {
			let r = u32::from_le_bytes(<[u8; 4]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<u32> for Record<'_> {
	fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = u32::from_le_bytes(<[u8; 4]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<u64> for Record<'_> {
	fn eq(&self, other: &u64) -> bool {
		if self.is_int() {
			let r = u64::from_le_bytes(<[u8; 8]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<u64> for Record<'_> {
	fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = u64::from_le_bytes(<[u8; 8]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<usize> for Record<'_> {
	fn eq(&self, other: &usize) -> bool {
		if self.is_int() {
			let r = usize::from_le_bytes(<[u8; 8]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<usize> for Record<'_> {
	fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = usize::from_le_bytes(<[u8; 8]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}







impl PartialEq<i16> for &Record<'_> {
	fn eq(&self, other: &i16) -> bool {
		if self.is_int() {
			let r = i16::from_le_bytes(<[u8; 2]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<i16> for &Record<'_> {
	fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = i16::from_le_bytes(<[u8; 2]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<i32> for &Record<'_> {
	fn eq(&self, other: &i32) -> bool {
		if self.is_int() {
			let r = i32::from_le_bytes(<[u8; 4]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<i32> for &Record<'_> {
	fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = i32::from_le_bytes(<[u8; 4]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<i64> for &Record<'_> {
	fn eq(&self, other: &i64) -> bool {
		if self.is_int() {
			let r = i64::from_le_bytes(<[u8; 8]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<i64> for &Record<'_> {
	fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = i64::from_le_bytes(<[u8; 8]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<u16> for &Record<'_> {
	fn eq(&self, other: &u16) -> bool {
		if self.is_int() {
			let r = u16::from_le_bytes(<[u8; 2]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<u16> for &Record<'_> {
	fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = u16::from_le_bytes(<[u8; 2]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<u32> for &Record<'_> {
	fn eq(&self, other: &u32) -> bool {
		if self.is_int() {
			let r = u32::from_le_bytes(<[u8; 4]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<u32> for &Record<'_> {
	fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = u32::from_le_bytes(<[u8; 4]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<u64> for &Record<'_> {
	fn eq(&self, other: &u64) -> bool {
		if self.is_int() {
			let r = u64::from_le_bytes(<[u8; 8]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<u64> for &Record<'_> {
	fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = u64::from_le_bytes(<[u8; 8]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

impl PartialEq<usize> for &Record<'_> {
	fn eq(&self, other: &usize) -> bool {
		if self.is_int() {
			let r = usize::from_le_bytes(<[u8; 8]>::try_from(self.raw_value()).unwrap());
			return r == *other
		}

		false
	}
}

impl PartialOrd<usize> for &Record<'_> {
	fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
		let raw_val = self.raw_value();
		if self.is_int() && !slice::is_zero(raw_val) {
			let r = usize::from_le_bytes(<[u8; 8]>::try_from(raw_val).unwrap());
		    if r > *other {
		    	Some(std::cmp::Ordering::Greater)
		    } else if r < *other { Some(std::cmp::Ordering::Less) }
		    else { Some(std::cmp::Ordering::Equal) }
		} else {
			None
		}
	}
}

