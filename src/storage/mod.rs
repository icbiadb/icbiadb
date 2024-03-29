//! Underlying storage of the Key-Value database implementation
//!

pub mod btreemap;
//pub mod ikv;

pub use btreemap::BTreeMap;
//pub use ikv::IndexedVec;

use crate::types::{BvObject, BvString};

pub trait KvInterface: std::default::Default + IntoIterator + Import + Export {
    type Key;
    type Value;
    type RefKey: ?Sized;

    fn with_capacity(cap: usize) -> Self;

    fn has_key(&self, key: &Self::RefKey) -> bool;

    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn indexes_len(&self) -> usize;

    fn insert(&mut self, key: Self::Key, value: Self::Value);
    fn insert_many(&mut self, records: Vec<(Self::Key, Self::Value)>);

    fn get(&self, key: &Self::RefKey) -> Option<&Self::Value>;
    fn get_mut(&mut self, key: &Self::RefKey) -> Option<&mut Self::Value>;

    fn remove(&mut self, key: &Self::RefKey) -> Option<Self::Value>;
}

pub trait Import {
    fn import(&mut self, from: Vec<(BvString, BvObject)>);
}

pub trait Export {
    fn export(&self) -> Vec<(BvString, BvObject)>;
}
