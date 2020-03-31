use crate::slice;
use crate::types::bv::ByteVec;


pub fn normalize_type_name(tn: &[u8]) -> &[u8] {
	match slice::strip_ref_symbols(tn) {
		// alloc::Alloc::String
		[97, 108, 108, 111, 99, 58, 58, 115, 116, 114, 105, 110, 103, 58, 58, 83, 116, 114, 105, 110, 103] => &[115, 116, 114],
		rest => rest
	}
}

pub fn serialize<T: ?Sized + serde::ser::Serialize>(t: &T) -> Vec<u8> {
	bincode::serialize(t).unwrap()
}

pub fn serialize_to_bytevec<T: ?Sized + serde::ser::Serialize>(t: &T) -> ByteVec {
	bincode::serialize(t).unwrap().into()	
}

pub fn deserialize<'a, T: ?Sized + serde::de::Deserialize<'a>>(t: &'a [u8]) -> T {
	bincode::deserialize(t).unwrap()
}

pub fn deserialize_bytevec<'a, T: ?Sized + serde::de::Deserialize<'a>>(t: &'a ByteVec) -> T {
	bincode::deserialize(t.as_slice()).unwrap()
}

pub fn from_utf8(b: &[u8]) -> &str {
	std::str::from_utf8(b).unwrap()
}







