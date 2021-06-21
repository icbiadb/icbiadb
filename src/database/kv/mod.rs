//! A Key-Value database implementation with multiple storage options.
//!
//! # Storage
//! * IndexedVec, first byte indexed vector, not recommended atm
//! * BTreeMap
//!
//! See [Storage](../../storage/index.html)

pub mod parser;
pub mod types;

use std::io::BufReader;

use crate::fio;
use crate::prelude::*;
use crate::storage::KvInterface;
use crate::types::*;
use crate::utils::{normalize_type_name, serialize, serialize_object};

pub fn mem<KV: KvInterface>() -> KvDb<KV> {
    KvDb {
        file_name: String::new(),
        records: KV::default(),
    }
}

pub fn create<KV: KvInterface<Key = BvString, Value = BvObject, RefKey = [u8]>>(
    file_name: &str,
) -> std::io::Result<KvDb<KV>> {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;

    let mut reader = fio::reader::Reader::new(BufReader::new(f));

    if reader.is_empty() {
        return Ok(KvDb {
            file_name: file_name.to_string(),
            records: KV::default(),
        });
    }

    Ok(KvDb::<KV> {
        file_name: file_name.to_string(),
        records: reader.read_kv_records()?,
    })
}

#[derive(Default)]
pub struct KvDb<KV: KvInterface> {
    pub file_name: String,
    pub records: KV,
}

impl<KV> KvDb<KV>
where
    KV: KvInterface<Key = BvString, Value = BvObject, RefKey = [u8]>,
    for<'a> &'a KV: IntoIterator<Item = (&'a BvString, &'a BvObject)>,
{
    /// Write the in-memory database to file
    ///
    pub fn commit(&self) -> std::io::Result<()> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file_name)?;

        let mut fio = fio::FileIO::new(f);
        fio.commit_kv_db(self)?;

        Ok(())
    }

    pub fn import(&mut self, data: Vec<(BvString, BvObject)>) {
        self.records.import(data);
    }

    pub fn export(&self) -> Vec<(BvString, BvObject)> {
        self.records.export()
    }

    /// Search keys for regex match
    #[cfg(feature = "regex_search")]
    pub fn key_regex<S: AsRef<str>>(&self, regex: S) -> Vec<(&BvString, &BvObject)> {
        let re = regex::bytes::Regex::new(regex.as_ref()).unwrap();
        self.filter(|(k, _)| re.is_match(k.as_slice()))
    }

    /// Search keys for RegexSet matches
    #[cfg(feature = "regex_search")]
    pub fn key_regexset<S: AsRef<str>>(&self, regex: &[S]) -> Vec<(&BvString, &BvObject)> {
        let set = regex::bytes::RegexSet::new(regex).unwrap();
        self.filter(|(k, _)| set.is_match(k.as_slice()))
    }

    /// Search string values for regex match
    #[cfg(feature = "regex_search")]
    pub fn value_regex<S: AsRef<str>>(&self, regex: S) -> Vec<(&BvString, &BvObject)> {
        let re = regex::bytes::Regex::new(regex.as_ref()).unwrap();
        self.filter(|(_, v)| v.is_str() && re.is_match(v.as_slice()))
    }

    /// Search string values for RegexSet matches
    #[cfg(feature = "regex_search")]
    pub fn value_regexset<S: AsRef<str>>(&self, regex: &[S]) -> Vec<(&BvString, &BvObject)> {
        let set = regex::bytes::RegexSet::new(regex).unwrap();
        self.filter(|(_, v)| v.is_str() && set.is_match(v.as_slice()))
    }

    /// Return the number of records stored in the database
    ///
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Check if the key already exists
    ///
    pub fn has_key<S: AsRef<str>>(&self, key: S) -> bool {
        self.records.has_key(key.as_ref().as_bytes())
    }

    /// Increment key by 1, isize is used by default if the key don't exists
    ///
    pub fn incr<S: AsRef<str>>(&mut self, key: S) {
        if self.has_key(key.as_ref()) {
            let v = self.get(key.as_ref()).unwrap().clone();
            if v.is_int() || v.is_uint() || v.is_float() {
                match v.type_name().as_str() {
                    "i8" => self.set(key, v.extract::<i8>() + 1),
                    "i16" => self.set(key, v.extract::<i16>() + 1),
                    "i32" => self.set(key, v.extract::<i32>() + 1),
                    "i64" => self.set(key, v.extract::<i64>() + 1),
                    "i128" => self.set(key, v.extract::<i128>() + 1),
                    "u8" => self.set(key, v.extract::<u8>() + 1),
                    "u16" => self.set(key, v.extract::<u16>() + 1),
                    "u32" => self.set(key, v.extract::<u32>() + 1),
                    "u64" => self.set(key, v.extract::<u64>() + 1),
                    "u128" => self.set(key, v.extract::<u128>() + 1),
                    "f32" => self.set(key, v.extract::<f32>() + 1.0),
                    "f64" => self.set(key, v.extract::<f64>() + 1.0),
                    _ => panic!("Something went wrong"),
                }
            }
        } else {
            self.set(key, 1 as isize);
        }
    }

    /// Increment key by T, isize is used by default if the key don't exists
    ///
    pub fn incr_by<S, T>(&mut self, key: S, val: T)
    where
        S: AsRef<str>,
        T: serde::ser::Serialize + serde::de::DeserializeOwned + std::ops::Add,
        <T as std::ops::Add>::Output: serde::ser::Serialize,
    {
        if self.has_key(key.as_ref()) {
            let v = self.get(key.as_ref()).unwrap().clone();
            if v.is_int() || v.is_uint() || v.is_float() {
                self.set(key, v.extract::<T>() + val);
            }
        } else {
            self.set(key, val);
        }
    }

    /// Decrement key by 1, isize is used by default if the key don't exists
    ///
    pub fn decr<S: AsRef<str>>(&mut self, key: S) {
        if self.has_key(key.as_ref()) {
            let v = self.get(key.as_ref()).unwrap().clone();
            if v.is_int() || v.is_uint() || v.is_float() {
                match v.type_name().as_str() {
                    "i8" => self.set(key, v.extract::<i8>() - 1),
                    "i16" => self.set(key, v.extract::<i16>() - 1),
                    "i32" => self.set(key, v.extract::<i32>() - 1),
                    "i64" => self.set(key, v.extract::<i64>() - 1),
                    "i128" => self.set(key, v.extract::<i128>() - 1),
                    "u8" => self.set(key, v.extract::<u8>() - 1),
                    "u16" => self.set(key, v.extract::<u16>() - 1),
                    "u32" => self.set(key, v.extract::<u32>() - 1),
                    "u64" => self.set(key, v.extract::<u64>() - 1),
                    "u128" => self.set(key, v.extract::<u128>() - 1),
                    "f32" => self.set(key, v.extract::<f32>() - 1.0),
                    "f64" => self.set(key, v.extract::<f64>() - 1.0),
                    _ => panic!("Something went wrong"),
                }
            }
        } else {
            self.set(key, 1 as isize);
        }
    }

    /// Decrement key by T, isize is used by default if the key don't exists
    ///
    pub fn decr_by<S, T>(&mut self, key: S, val: T)
    where
        S: AsRef<str>,
        T: serde::ser::Serialize + serde::de::DeserializeOwned + std::ops::Sub,
        <T as std::ops::Sub>::Output: serde::ser::Serialize,
    {
        if self.has_key(key.as_ref()) {
            let v = self.get(key.as_ref()).unwrap().clone();
            if v.is_int() || v.is_uint() || v.is_float() {
                self.set(key, v.extract::<T>() - val);
            }
        } else {
            self.set(key, val);
        }
    }

    /// Replace value with mem::replace and return the old value
    ///
    pub fn swap<S: AsRef<str>, T: serde::Serialize>(&mut self, key: S, value: T) -> BvObject {
        let new_obj = serialize_object(&value);
        let old_obj = self.records.get_mut(key.as_ref().as_bytes()).unwrap();

        if new_obj.type_name() == old_obj.type_name() && new_obj.raw().len() == old_obj.raw().len()
        {
            return std::mem::replace(old_obj, new_obj);
        }

        panic!("Not same type or equal length")
    }

    /// Set a key to value T
    ///
    pub fn set<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, key: S, value: T) {
        if !self.has_key(key.as_ref()) {
            // Create new
            let value = serialize_object(&value);
            assert!(!key.as_ref().is_empty() && !value.type_name().is_empty());

            self.records
                .insert(key.as_ref().as_bytes().to_vec().into(), value);
        } else {
            // Update
            let old = self
                .records
                .get(key.as_ref().as_bytes())
                .unwrap()
                .type_name();
            let new = serialize_object(&value);

            if old.inner().len() == new.inner().len() {
                *self.records.get_mut(key.as_ref().as_bytes()).unwrap() = new;
            } else {
                self.del(key.as_ref());
                self.records
                    .insert(key.as_ref().as_bytes().to_vec().into(), new);
            }
        }
    }

    /// Set a key to value T with type name S
    ///
    pub fn set_as<S: AsRef<str>, T: Sized + serde::ser::Serialize>(
        &mut self,
        key: S,
        t: S,
        value: T,
    ) {
        let value = BvObject::from_raw(
            normalize_type_name(t.as_ref().as_bytes()).to_vec(),
            serialize(&value),
        );
        assert!(!key.as_ref().is_empty() && !value.type_name().is_empty());
        self.set(key, value);
    }

    /// Set a key to Vec<u8> with type name S
    ///
    pub fn set_raw<S: AsRef<str>>(&mut self, key: S, type_name: S, value: Vec<u8>) {
        let value = BvObject::from_raw(
            normalize_type_name(type_name.as_ref().as_bytes()).to_vec(),
            value,
        );
        assert!(!key.as_ref().is_empty() && !value.type_name().is_empty());
        self.set(key, value);
    }

    pub fn set_many<S: AsRef<str>, T: Sized + serde::ser::Serialize>(
        &mut self,
        mut values: Vec<(S, T)>,
    ) {
        for (k, v) in values.drain(..) {
            //let v = serialize_object(&v);
            //assert!(k.as_ref().len() > 0 && v.type_name().len() > 0);
            self.set(k, v);
        }
    }

    pub fn set_many_as<S: AsRef<str>, T: Sized + serde::ser::Serialize>(
        &mut self,
        values: Vec<(S, S, T)>,
    ) {
        for (k, t, v) in values {
            let v = BvObject::from_raw(
                normalize_type_name(t.as_ref().as_bytes()).to_vec(),
                serialize(&v),
            );
            assert!(!k.as_ref().is_empty() && !v.type_name().is_empty());
            self.set(k, v);
        }
    }

    /// Retrieve a BvObject
    ///
    pub fn get<S: AsRef<str>>(&self, key: S) -> Option<&BvObject> {
        self.records.get(key.as_ref().as_bytes())
    }

    /// Retrieve and deserialize a value to T
    ///
    pub fn get_value<T: serde::de::DeserializeOwned>(&self, key: &str) -> T {
        self.records.get(key.as_bytes()).unwrap().extract()
    }

    /// Retrieve a value as BvTuple
    ///
    pub fn get_tuple<S: AsRef<str>>(&mut self, key: S) -> Option<BvTuple> {
        match self.records.get_mut(key.as_ref().as_bytes()) {
            Some(t) => Some(BvTuple::from(t)),
            None => None,
        }
    }

    /// Retrieve a value as BvStr
    ///
    pub fn get_str<S: AsRef<str>>(&mut self, key: S) -> BvStr {
        BvStr::from_bvobject(self.records.get_mut(key.as_ref().as_bytes()).unwrap())
    }

    /// Delete key and return the deleted object
    ///
    pub fn del<S: AsRef<str>>(&mut self, key: S) -> BvObject {
        self.records.remove(key.as_ref().as_bytes())
    }
}

impl<KV> BytesFilter for KvDb<KV>
where
    KV: KvInterface<Key = BvString, Value = BvObject>,
    for<'a> &'a KV: IntoIterator<Item = (&'a BvString, &'a BvObject)>,
{
    fn filter<F>(&self, cb: F) -> Vec<(&BvString, &BvObject)>
    where
        F: Fn((&BvString, &BvObject)) -> bool,
    {
        (&self.records).into_iter().filter(|t| cb(*t)).collect()
    }
}

impl<KV: KvInterface<Key = BvString, Value = BvObject>> BytesSearch for KvDb<KV>
where
    KV: KvInterface<Key = BvString, Value = BvObject>,
    for<'a> &'a KV: IntoIterator<Item = (&'a BvString, &'a BvObject)>,
{
    fn starts_with<S: AsRef<str>>(&self, key_part: S) -> Vec<(&BvString, &BvObject)> {
        let k_part = key_part.as_ref().as_bytes();
        (&self.records)
            .into_iter()
            .filter_map(|(k, v)| {
                if k.starts_with(k_part) {
                    return Some((k, v));
                }

                None
            })
            .collect()
    }

    fn ends_with<S: AsRef<str>>(&self, key_part: S) -> Vec<(&BvString, &BvObject)> {
        (&self.records)
            .into_iter()
            .filter_map(|(k, v)| {
                if k.ends_with(key_part.as_ref().as_bytes()) {
                    return Some((k, v));
                }

                None
            })
            .collect::<Vec<_>>()
    }

    fn contains<S: AsRef<str>>(&self, key_part: S) -> Vec<(&BvString, &BvObject)> {
        (&self.records)
            .into_iter()
            .filter_map(|(k, v)| {
                if k.contains(key_part.as_ref()) {
                    return Some((k, v));
                }

                None
            })
            .collect::<Vec<_>>()
    }
}
