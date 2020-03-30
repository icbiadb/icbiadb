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

