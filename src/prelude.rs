use crate::types::*;


pub trait BytesFilter {
	fn filter<F>(&self, cb: F) -> Vec<Record> where F: Fn(&Record) -> bool;
}

pub trait BytesSearch {
	fn starts_with<S: AsRef<str>>(&self, key_part: S) -> Vec<Record>;
	fn ends_with<S: AsRef<str>>(&self, key_part: S) -> Vec<Record>;
	fn contains<S: AsRef<str>>(&self, key_part: S) -> Vec<Record>;
}

pub trait RecordRead {
	fn key(&self) -> &str;
	fn raw_key(&self) -> &[u8];
	fn type_name(&self) -> &str;
	fn raw_type_name(&self) -> &[u8];
	fn value<T: ?Sized + serde::de::DeserializeOwned>(&self) -> T;
	fn raw_value(&self) -> &[u8];
	fn as_tuple(&self) -> (&[u8], &[u8], &[u8]);
	fn to_tuple(&self) -> (Vec<u8>, Vec<u8>, Vec<u8>);
}

pub trait RecordWrite {
	fn set_value<T: ?Sized + serde::ser::Serialize>(&mut self, v: &T);
}
