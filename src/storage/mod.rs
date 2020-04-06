
pub mod ikvstorage;


pub use ikvstorage::IndexedKvStorage;



pub trait KvInterface: std::default::Default + IntoIterator {
	type Key;
	type Value;
	type RefKey: ?Sized;

	fn has_key(&self, key: &Self::RefKey) -> bool;
	fn len(&self) -> usize;
	fn indexes_len(&self) -> usize;

	fn insert(&mut self, key: Self::Key, value: Self::Value);
	fn insert_many(&mut self, records: Vec<(Self::Key, Self::Value)>);

	fn get(&self, key: &Self::RefKey) -> &Self::Value;
	fn get_mut(&mut self, key: &Self::RefKey) -> &mut Self::Value;

	fn remove(&mut self, key: &Self::RefKey) -> Self::Value;
}

