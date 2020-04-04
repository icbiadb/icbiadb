use crate::types::bv::{BvStr, BvObject};


pub trait BytesFilter {
	fn filter<F>(&self, cb: F) -> Vec<(BvStr, &BvObject)> where F: Fn((BvStr, &BvObject)) -> bool;
}

pub trait BytesSearch {
	fn starts_with<S: AsRef<str>>(&self, key_part: S) -> Vec<(BvStr, &BvObject)>;
	fn ends_with<S: AsRef<str>>(&self, key_part: S) -> Vec<(BvStr, &BvObject)>;
	fn contains<S: AsRef<str>>(&self, key_part: S) -> Vec<(BvStr, &BvObject)>;
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


