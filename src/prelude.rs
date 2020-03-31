use crate::types::record::Record;
use crate::types::bv::{BvString, ByteVec};


pub trait BytesFilter {
	fn filter<F>(&self, cb: F) -> Vec<Record> where F: Fn(&Record) -> bool;
}

pub trait BytesSearch {
	fn starts_with<S: AsRef<str>>(&self, key_part: S) -> Vec<Record>;
	fn ends_with<S: AsRef<str>>(&self, key_part: S) -> Vec<Record>;
	fn contains<S: AsRef<str>>(&self, key_part: S) -> Vec<Record>;
}

pub trait RecordRead {
	fn key(&self) -> &BvString;
	fn type_name(&self) -> &BvString;
	fn value<T: ?Sized + serde::de::DeserializeOwned>(&self) -> T;
	fn raw_value(&self) -> &ByteVec;
	fn as_tuple(&self) -> (&BvString, &BvString, &ByteVec);
	fn to_tuple(&self) -> (BvString, BvString, ByteVec);
}

pub trait RecordWrite {
	fn set_value<T: ?Sized + serde::ser::Serialize>(&mut self, v: &T);
}



pub trait BvStartsWith<T: ?Sized> {
	fn starts_with(&self, other: T) -> bool;
}

pub trait BvEndsWith<T: ?Sized> {
	fn ends_with(&self, other: T) -> bool;
}

pub trait BvContains<T: ?Sized> {
	fn contains(&self, s: T) -> bool;
}


