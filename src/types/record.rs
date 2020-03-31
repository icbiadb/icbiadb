use super::bv::{BvString, ByteVec};
use crate::prelude::{BvContains, BvStartsWith, BvEndsWith};

use crate::{
	prelude::*,
	utils::*,
};



// TODO
// Cache deserialization
#[derive(Debug)]
pub struct Record<'a> {
	key: &'a BvString,
	type_name: &'a BvString,
	value: &'a ByteVec,
}

impl<'a> Record<'a> {
	pub fn new(key: &'a BvString, type_name: &'a BvString, value: &'a ByteVec) -> Self {
		Record {
			key: key,
			type_name: type_name,
			value: value,
		}
	}

	pub fn is_str(&self) -> bool {
		self.type_name == "str"
	}

	pub fn is_int(&self) -> bool {
		match self.type_name().as_slice() {
			// i8-i64
			[105, 56] => true,
			[105, 49, 54] => true,
			[105, 51, 50] => true,
			[105, 54, 52] => true,
			_ => false
		}
	}

	pub fn is_uint(&self) -> bool {
		match self.type_name().as_slice() {
			// u8-u64
			[117, 56] => true,
			[117, 49, 54] => true,
			[117, 51, 50] => true,
			[117, 54, 52] => true,
			_ => false
		}
	}

	pub fn is_float(&self) -> bool {
		match self.type_name().as_slice() {
			// f32-f64
			[102, 51, 50] => true,
			[102, 54, 52] => true,
			_ => false
		}
	}
}

impl RecordRead for Record<'_> {
	fn key(&self) -> &BvString {
		&self.key
	}

	fn type_name(&self) -> &BvString {	
		&self.type_name
	}

	fn value<T: ?Sized + serde::de::DeserializeOwned>(&self) -> T {
		deserialize_bytevec(&self.value)
	}

	fn raw_value(&self) -> &ByteVec {
		self.value
	}

	fn as_tuple(&self) -> (&BvString, &BvString, &ByteVec) {
		(self.key, self.type_name, self.value)
	}

	fn to_tuple(&self) -> (BvString, BvString, ByteVec) {
		(self.key.to_vec().into(), self.type_name.to_vec().into(), self.value.to_vec().into())
	}
}


impl BvStartsWith<&String> for Record<'_> {
	fn starts_with(&self, other: &String) -> bool {
		self.is_str() && self.raw_value().starts_with(other.as_bytes())
	}
}

impl BvStartsWith<&[u8]> for Record<'_> {
	fn starts_with(&self, other: &[u8]) -> bool {
		self.is_str() && self.raw_value().starts_with(other)
	}
}

impl BvStartsWith<&str> for Record<'_> {
	fn starts_with(&self, other: &str) -> bool {
		self.is_str() && self.raw_value().starts_with(other)
	}
}

impl BvContains<&String> for Record<'_> {
	fn contains(&self, other: &String) -> bool {
		self.is_str() && self.raw_value().contains(other.as_bytes())
	}
}

impl BvContains<&[u8]> for Record<'_> {
	fn contains(&self, other: &[u8]) -> bool {
		self.is_str() && self.raw_value().contains(other)
	}
}

impl BvContains<&str> for Record<'_> {
	fn contains(&self, other: &str) -> bool {
		self.is_str() && self.raw_value().contains(other)
	}
}

impl BvEndsWith<&String> for Record<'_> {
	fn ends_with(&self, other: &String) -> bool {
		self.is_str() && self.raw_value().ends_with(other.as_bytes())
	}
}

impl BvEndsWith<&[u8]> for Record<'_> {
	fn ends_with(&self, other: &[u8]) -> bool {
		self.is_str() && self.raw_value().ends_with(other)
	}
}

impl BvEndsWith<&str> for Record<'_> {
	fn ends_with(&self, other: &str) -> bool {
		self.is_str() && self.raw_value().ends_with(other)
	}
}


impl<'a> std::convert::From<&'a (BvString, BvString, ByteVec)> for Record<'a> {
	fn from(tuple: &'a (BvString, BvString, ByteVec)) -> Self {
		let (k, t, v) = tuple;
		Record::new(k, t, v)
	}
}

impl std::fmt::Display for Record<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Record {{ key: {:?}, type_name: {}, value: {:?} }}",
			std::str::from_utf8(self.key().as_slice()).unwrap(),
			std::str::from_utf8(self.type_name().as_slice()).unwrap(), 
			self.value
			)
	}
}















impl PartialEq<&str> for Record<'_> {
	fn eq(&self, other: &&str) -> bool {
		self.is_str() && self.raw_value() == other.as_bytes()
	}
}

impl PartialEq<String> for Record<'_> {
	fn eq(&self, other: &String) -> bool {
		self.is_str() && self.raw_value() == other.as_bytes()
	}
}

// TODO
// Byte comparison and such instead



impl PartialEq<i16> for Record<'_> {
	fn eq(&self, other: &i16) -> bool {
		self.is_int() && self.raw_value().as_i16() == *other
	}
}

impl PartialOrd<i16> for Record<'_> {
	fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
		if !self.is_int() { return None }

		let value = self.raw_value().as_i16();
		return if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<i32> for Record<'_> {
	fn eq(&self, other: &i32) -> bool {
		self.is_int() && self.raw_value().as_i32() == *other
	}
}

impl PartialOrd<i32> for Record<'_> {
	fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
		if !self.is_int() { return None }

		let value = self.raw_value().as_i32();
		return if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<i64> for Record<'_> {
	fn eq(&self, other: &i64) -> bool {
		self.is_int() && self.raw_value().as_i64() == *other
	}
}

impl PartialOrd<i64> for Record<'_> {
	fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
		if !self.is_int() { return None }

		let value = self.raw_value().as_i64();
		return if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<i128> for Record<'_> {
	fn eq(&self, other: &i128) -> bool {
		self.is_uint() && self.raw_value().as_i128() == *other
	}
}

impl PartialOrd<i128> for Record<'_> {
	fn partial_cmp(&self, other: &i128) -> Option<std::cmp::Ordering> {
		if !self.is_uint() { return None }

		let value = self.raw_value().as_i128();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u16> for Record<'_> {
	fn eq(&self, other: &u16) -> bool {
		self.is_uint() && self.raw_value().as_u16() == *other
	}
}

impl PartialOrd<u16> for Record<'_> {
	fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
		if !self.is_uint() { return None }

		let value = self.raw_value().as_u16();
		return if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u32> for Record<'_> {
	fn eq(&self, other: &u32) -> bool {
		self.is_uint() && self.raw_value().as_u32() == *other
	}
}

impl PartialOrd<u32> for Record<'_> {
	fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
		if !self.is_uint() { return None }

		let value = self.raw_value().as_u32();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u64> for Record<'_> {
	fn eq(&self, other: &u64) -> bool {
		self.is_uint() && self.raw_value().as_u64() == *other
	}
}

impl PartialOrd<u64> for Record<'_> {
	fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
		if !self.is_uint() { return None }

		let value = self.raw_value().as_u64();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<usize> for Record<'_> {
	fn eq(&self, other: &usize) -> bool {
		self.is_uint() && self.raw_value().as_usize() == *other
	}
}

impl PartialOrd<usize> for Record<'_> {
	fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
		if !self.is_uint() { return None }

		let value = self.raw_value().as_usize();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u128> for Record<'_> {
	fn eq(&self, other: &u128) -> bool {
		self.is_uint() && self.raw_value().as_u128() == *other
	}
}

impl PartialOrd<u128> for Record<'_> {
	fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
		if !self.is_uint() { return None }

		let value = self.raw_value().as_u128();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}





impl PartialEq<i16> for &Record<'_> {
	fn eq(&self, other: &i16) -> bool {
		self.is_int() && self.raw_value().as_i16() == *other
	}
}

impl PartialOrd<i16> for &Record<'_> {
	fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
		if !self.is_int() { return None }

		let value = self.raw_value().as_i16();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<i32> for &Record<'_> {
	fn eq(&self, other: &i32) -> bool {
		self.is_int() && self.raw_value().as_i32() == *other
	}
}

impl PartialOrd<i32> for &Record<'_> {
	fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
		if !self.is_int() { return None }

		let value = self.raw_value().as_i32();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<i64> for &Record<'_> {
	fn eq(&self, other: &i64) -> bool {
		self.is_int() && self.raw_value().as_i64() == *other
	}
}

impl PartialOrd<i64> for &Record<'_> {
	fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
		if !self.is_int() { return None }

		let value = self.raw_value().as_i64();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<i128> for &Record<'_> {
	fn eq(&self, other: &i128) -> bool {
		self.is_int() && self.raw_value().as_i128() == *other
	}
}

impl PartialOrd<i128> for &Record<'_> {
	fn partial_cmp(&self, other: &i128) -> Option<std::cmp::Ordering> {
		if !self.is_int() { return None }

		let value = self.raw_value().as_i128();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u16> for &Record<'_> {
	fn eq(&self, other: &u16) -> bool {
		self.is_uint() && self.raw_value().as_u16() == *other
	}
}

impl PartialOrd<u16> for &Record<'_> {
	fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
		if !self.is_uint() { return None }

		let value = self.raw_value().as_u16();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u32> for &Record<'_> {
	fn eq(&self, other: &u32) -> bool {
		self.is_uint() && self.raw_value().as_u32() == *other
	}
}

impl PartialOrd<u32> for &Record<'_> {
	fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
		if !self.is_uint() { return None }

		let value = self.raw_value().as_u32();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u64> for &Record<'_> {
	fn eq(&self, other: &u64) -> bool {
		self.is_uint() && self.raw_value().as_u64() == *other
	}
}

impl PartialOrd<u64> for &Record<'_> {
	fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
		if !self.is_uint() { return None }

		let value = self.raw_value().as_u64();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<u128> for &Record<'_> {
	fn eq(&self, other: &u128) -> bool {
		self.is_int() && self.raw_value().as_u128() == *other
	}
}

impl PartialOrd<u128> for &Record<'_> {
	fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
		if !self.is_int() { return None }

		let value = self.raw_value().as_u128();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<usize> for &Record<'_> {
	fn eq(&self, other: &usize) -> bool {
		self.is_uint() && self.raw_value().as_usize() == *other
	}
}

impl PartialOrd<usize> for &Record<'_> {
	fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
		if !self.is_uint() { return None }

		let value = self.raw_value().as_usize();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<f32> for &Record<'_> {
	fn eq(&self, other: &f32) -> bool {
		self.is_float() && self.raw_value().as_f32() == *other
	}
}

impl PartialOrd<f32> for &Record<'_> {
	fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
		if !self.is_float() { return None }

		let value = self.raw_value().as_f32();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl PartialEq<f64> for &Record<'_> {
	fn eq(&self, other: &f64) -> bool {
		self.is_float() && self.raw_value().as_f64() == *other
	}
}

impl PartialOrd<f64> for &Record<'_> {
	fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
		if !self.is_float() { return None }

		let value = self.raw_value().as_f64();
		if value > *other {
			Some(std::cmp::Ordering::Greater)
		} else if value < *other {
			Some(std::cmp::Ordering::Less) 
		} else { 
			Some(std::cmp::Ordering::Equal)
		}
	}
}


