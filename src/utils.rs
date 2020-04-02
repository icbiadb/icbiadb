use crate::slice;
use crate::types::bv::{ByteVec, BvObject};


pub fn normalize_type_name(tn: &[u8]) -> &[u8] {
	// Assume bincode casts f32 to f64
	match slice::strip_ref_symbols(tn) {
		// f32 -> f64
		[102, 51, 50] => &[102, 54, 52],
		// isize -> i64
		[105, 115, 105, 122, 101] => &[105, 54, 52],
		// usize -> u64
		[117, 115, 105, 122, 101] => &[117, 54, 52],
		// alloc::Alloc::String -> str
		[97, 108, 108, 111, 99, 58, 58, 115, 116, 114, 105, 110, 103, 58, 58, 83, 116, 114, 105, 110, 103] => &[115, 116, 114],
		rest => rest
	}
}

pub fn serialize<T: ?Sized + serde::ser::Serialize>(o: &T) -> Vec<u8> {
	bincode::serialize(o).unwrap()
}

pub fn serialize_object<T: ?Sized + serde::ser::Serialize>(o: &T) -> BvObject {
	BvObject::from(o)
}

pub fn serialize_to_bytevec<T: ?Sized + serde::ser::Serialize>(t: &T) -> ByteVec {
	bincode::serialize(t).unwrap().into()	
}

pub fn deserialize<'a, T: ?Sized + serde::de::Deserialize<'a>>(t: &'a [u8]) -> T {
	bincode::deserialize(t).unwrap()
}

pub fn deserialize_object<'a, T: ?Sized + serde::de::Deserialize<'a>>(t: &'a BvObject) -> T {
	bincode::deserialize(t.as_slice()).unwrap()
}

pub fn deserialize_bytevec<'a, T: ?Sized + serde::de::Deserialize<'a>>(t: &'a ByteVec) -> T {
	bincode::deserialize(t.as_slice()).unwrap()
}

pub fn from_utf8(b: &[u8]) -> &str {
	std::str::from_utf8(b).unwrap()
}







