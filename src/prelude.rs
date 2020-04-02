use crate::types::bv::{BvString, BvObject};


pub trait BytesFilter {
	fn filter<F>(&self, cb: F) -> Vec<&(BvString, BvObject)> where F: Fn(&(BvString, BvObject)) -> bool;
}

pub trait BytesSearch {
	fn starts_with<S: AsRef<str>>(&self, key_part: S) -> Vec<(&BvString, &BvObject)>;
	fn ends_with<S: AsRef<str>>(&self, key_part: S) -> Vec<(&BvString, &BvObject)>;
	fn contains<S: AsRef<str>>(&self, key_part: S) -> Vec<(&BvString, &BvObject)>;
}

pub trait RecordRead {
	fn key(&self) -> &BvString;
	fn value<T: ?Sized + serde::de::DeserializeOwned>(&self) -> T;
	fn raw_value(&self) -> &BvObject;
	fn as_tuple(&self) -> (&BvString, &BvObject);
	fn to_tuple(&self) -> (BvString, BvObject);
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


