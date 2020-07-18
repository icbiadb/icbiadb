use super::KvInterface;
use crate::types::{BvObject, BvString};
use std::borrow::Borrow;

#[derive(Default, Clone)]
pub struct IndexVec<I: Eq, V>(Vec<(I, V)>);

impl<I: Eq, V> IndexVec<I, V> {
    fn key_index<Q>(&self, b: &Q) -> Option<usize>
    where
        I: Borrow<Q> + PartialEq<Q>,
        Q: Eq + ?Sized,
    {
        for (i, (index, _)) in self.0.iter().enumerate() {
            if index == b {
                return Some(i);
            }
        }

        None
    }

    fn has_index<Q>(&self, b: &Q) -> bool
    where
        I: Borrow<Q> + PartialEq<Q>,
        Q: Eq + ?Sized,
    {
        for (index, _) in self.0.iter() {
            if index == b {
                return true;
            }
        }

        false
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        I: Borrow<Q> + PartialEq<Q>,
        Q: Eq + ?Sized,
    {
        for (index, value) in &self.0 {
            if index == key {
                return Some(value);
            }
        }

        None
    }

    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        I: Borrow<Q> + PartialEq<Q>,
        Q: Eq + ?Sized,
    {
        for (index, value) in &mut self.0 {
            if index == key {
                return Some(value);
            }
        }

        None
    }

    fn push(&mut self, record: (I, V)) {
        self.0.push(record)
    }

    fn remove<Q>(&mut self, key: &Q) -> V
    where
        I: Borrow<Q> + PartialEq<Q>,
        Q: Eq + ?Sized,
    {
        self.0.remove(self.key_index(key).unwrap()).1
    }
}

impl<I: Eq, V> std::ops::Deref for IndexVec<I, V> {
    type Target = Vec<(I, V)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I, V, Q> std::ops::Index<&Q> for IndexVec<I, V>
where
    I: Eq + Borrow<Q> + PartialEq<Q>,
    Q: Eq + ?Sized + std::fmt::Debug,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("Failed to find index {:?}", index))
    }
}

impl<I, V, Q> std::ops::IndexMut<&Q> for IndexVec<I, V>
where
    I: Eq + Borrow<Q> + PartialEq<Q> + std::fmt::Debug,
    Q: Eq + ?Sized + std::fmt::Debug,
{
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.get_mut(index)
            .unwrap_or_else(|| panic!("Failed to find index {:?}", index))
    }
}

impl<I: Eq, V> std::ops::Index<usize> for IndexVec<I, V> {
    type Output = (I, V);

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<I: Eq, V> std::ops::IndexMut<usize> for IndexVec<I, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<I: Eq, V> std::convert::From<Vec<(I, V)>> for IndexVec<I, V> {
    fn from(other: Vec<(I, V)>) -> Self {
        IndexVec(other)
    }
}

#[derive(Default, Clone)]
pub struct IndexedKvStorage {
    inner: IndexVec<u8, IndexVec<BvString, BvObject>>,
}

impl KvInterface for IndexedKvStorage {
    type Key = BvString;
    type Value = BvObject;
    type RefKey = [u8];

    fn indexes_len(&self) -> usize {
        self.inner.len()
    }

    fn has_key(&self, key: &[u8]) -> bool {
        if self.inner.has_index(&key[0]) && self.inner[&key[0]].has_index(key) {
            return true;
        }

        false
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn len(&self) -> usize {
        let mut length = 0;
        for (_, v) in self.inner.iter() {
            length += v.len();
        }

        length
    }

    fn insert(&mut self, key: BvString, v: BvObject) {
        if !self.inner.has_index(&key[0]) {
            self.inner.push((key[0], vec![(key, v)].into()));
            return;
        }

        self.inner[&key[0]].push((key, v));
    }

    fn insert_many(&mut self, _records: Vec<(BvString, BvObject)>) {
        // TODO
        // Filter by first key, then extend
        /*
        _records.iter()
            .for_each(||)
        */
    }

    fn get(&self, key: &[u8]) -> &BvObject {
        &self.inner[&key[0]][key]
    }

    fn get_mut(&mut self, key: &[u8]) -> &mut BvObject {
        &mut self.inner[&key[0]][key]
    }

    fn remove(&mut self, key: &[u8]) -> BvObject {
        self.inner[&key[0]].remove(key)
    }
}

impl IntoIterator for IndexedKvStorage {
    type Item = (BvString, BvObject);
    type IntoIter = IndexedMKvStorageIter;

    fn into_iter(self) -> Self::IntoIter {
        IndexedMKvStorageIter {
            inner: self,
            key_part: 0,
            index: 0,
        }
    }
}

pub struct IndexedMKvStorageIter {
    inner: IndexedKvStorage,
    key_part: usize,
    index: usize,
}

impl std::iter::Iterator for IndexedMKvStorageIter {
    type Item = (BvString, BvObject);

    fn next(&mut self) -> Option<Self::Item> {
        let mut item = None;

        if self.inner.indexes_len() > 0 {
            if self.index < self.inner[self.key_part].1.len() {
                item = Some(self.inner[self.key_part].1[self.index].clone());
                self.index += 1;
            } else {
                self.key_part += 1;
                self.index = 0;

                if self.key_part == self.inner.indexes_len() {
                    return None;
                }

                item = Some(self.inner[self.key_part].1[self.index].clone());
                self.index += 1;
            }
        }

        item
    }
}

impl<'a> IntoIterator for &'a IndexedKvStorage {
    type Item = &'a (BvString, BvObject);
    type IntoIter = IndexedKvStorageIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IndexedKvStorageIter {
            inner: self,
            key_part: 0,
            index: 0,
        }
    }
}

impl std::ops::Index<usize> for IndexedKvStorage {
    type Output = (u8, IndexVec<BvString, BvObject>);

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl std::ops::IndexMut<usize> for IndexedKvStorage {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

pub struct IndexedKvStorageIter<'a> {
    inner: &'a IndexedKvStorage,
    key_part: usize,
    index: usize,
}

impl<'a> std::iter::Iterator for IndexedKvStorageIter<'a> {
    type Item = &'a (BvString, BvObject);

    fn next(&mut self) -> Option<Self::Item> {
        let mut item = None;

        if self.inner.indexes_len() > 0 {
            if self.index < self.inner[self.key_part].1.len() {
                item = Some(&self.inner[self.key_part].1[self.index]);
                self.index += 1;
            } else {
                self.key_part += 1;
                self.index = 0;

                if self.key_part == self.inner.indexes_len() {
                    return None;
                }

                item = Some(&self.inner[self.key_part].1[self.index]);
                self.index += 1;
            }
        }

        item
    }
}
