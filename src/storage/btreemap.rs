use super::KvInterface;
use crate::types::{BvObject, BvString};
use std::collections::BTreeMap as btmp;

#[derive(Default)]
pub struct BTreeMap(btmp<BvString, BvObject>);

impl KvInterface for BTreeMap {
    type Key = BvString;
    type Value = BvObject;
    type RefKey = [u8];

    fn with_capacity(_: usize) -> Self {
        unimplemented!("BTreemap don't support with_capacity")
    }

    fn has_key(&self, key: &[u8]) -> bool {
        self.0.contains_key(key)
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn indexes_len(&self) -> usize {
        unimplemented!()
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) {
        self.0.insert(key, value);
    }

    fn insert_many(&mut self, _records: Vec<(Self::Key, Self::Value)>) {
        unimplemented!()
    }

    fn get(&self, key: &Self::RefKey) -> Option<&Self::Value> {
        self.0.get(key)
    }

    fn get_mut(&mut self, key: &Self::RefKey) -> Option<&mut Self::Value> {
        self.0.get_mut(key)
    }

    fn remove(&mut self, key: &Self::RefKey) -> Self::Value {
        self.0.remove(key).unwrap()
    }
}

impl IntoIterator for BTreeMap {
    type Item = (BvString, BvObject);
    type IntoIter = std::collections::btree_map::IntoIter<BvString, BvObject>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a BTreeMap {
    type Item = (&'a BvString, &'a BvObject);
    type IntoIter = BTreeMapIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BTreeMapIntoIter {
            inner: self.0.iter(),
        }
    }
}

pub struct BTreeMapIntoIter<'a> {
    inner: std::collections::btree_map::Iter<'a, BvString, BvObject>,
}

impl<'a> std::iter::Iterator for BTreeMapIntoIter<'a> {
    type Item = (&'a BvString, &'a BvObject);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
