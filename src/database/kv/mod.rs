pub mod parser;
pub mod types;

use crate::prelude::*;
use crate::storage::KvInterface;
use crate::types::*;
use crate::utils::{normalize_type_name, serialize, serialize_object};

#[derive(Default)]
pub struct KvDb<KV: KvInterface<Key = BvString, Value = BvObject, RefKey = [u8]>> {
    pub records: KV,
}

impl<KV> KvDb<KV>
where
    KV: KvInterface<Key = BvString, Value = BvObject, RefKey = [u8]>,
    for<'a> &'a KV: IntoIterator<Item = &'a (BvString, BvObject)>,
{
    /// Search keys for regex match
    #[cfg(feature = "regex_search")]
    pub fn key_regex<S: AsRef<str>>(&self, regex: S) -> Vec<&(BvString, BvObject)> {
        let re = regex::bytes::Regex::new(regex.as_ref()).unwrap();
        self.filter(|(k, _)| re.is_match(k.as_slice()))
    }

    /// Search keys for RegexSet matches
    #[cfg(feature = "regex_search")]
    pub fn key_regexset<S: AsRef<str>>(&self, regex: &[S]) -> Vec<&(BvString, BvObject)> {
        let set = regex::bytes::RegexSet::new(regex).unwrap();
        self.filter(|(k, _)| set.is_match(k.as_slice()))
    }

    /// Search string values for regex match
    #[cfg(feature = "regex_search")]
    pub fn value_regex<S: AsRef<str>>(&self, regex: S) -> Vec<&(BvString, BvObject)> {
        let re = regex::bytes::Regex::new(regex.as_ref()).unwrap();
        self.filter(|(_, v)| v.is_str() && re.is_match(v.as_slice()))
    }

    /// Search string values for RegexSet matches
    #[cfg(feature = "regex_search")]
    pub fn value_regexset<S: AsRef<str>>(&self, regex: &[S]) -> Vec<&(BvString, BvObject)> {
        let set = regex::bytes::RegexSet::new(regex).unwrap();
        self.filter(|(_, v)| v.is_str() && set.is_match(v.as_slice()))
    }

    pub fn has_key<S: AsRef<str>>(&self, key: S) -> bool {
        self.records.has_key(key.as_ref().as_bytes())
    }

    pub fn incr<S: AsRef<str>>(&mut self, key: S) {
        if self.has_key(key.as_ref()) {
            let v = self.get(key.as_ref()).clone();
            if v.is_int() || v.is_uint() || v.is_float() {
                match v.type_name().as_str() {
                    "i8" => self.update(key, v.extract::<i8>() + 1),
                    "i16" => self.update(key, v.extract::<i16>() + 1),
                    "i32" => self.update(key, v.extract::<i32>() + 1),
                    "i64" => self.update(key, v.extract::<i64>() + 1),
                    "i128" => self.update(key, v.extract::<i128>() + 1),
                    "u8" => self.update(key, v.extract::<u8>() + 1),
                    "u16" => self.update(key, v.extract::<u16>() + 1),
                    "u32" => self.update(key, v.extract::<u32>() + 1),
                    "u64" => self.update(key, v.extract::<u64>() + 1),
                    "u128" => self.update(key, v.extract::<u128>() + 1),
                    "f32" => self.update(key, v.extract::<f32>() + 1.0),
                    "f64" => self.update(key, v.extract::<f64>() + 1.0),
                    _ => panic!("Something went wrong"),
                }
            }
        } else {
            self.set(key, 1 as isize);
        }
    }

    pub fn incr_by<S, T>(&mut self, key: S, val: T)
    where
        S: AsRef<str>,
        T: serde::ser::Serialize + serde::de::DeserializeOwned + std::ops::Add,
        <T as std::ops::Add>::Output: serde::ser::Serialize,
    {
        if self.has_key(key.as_ref()) {
            let v = self.get(key.as_ref()).clone();
            if v.is_int() || v.is_uint() || v.is_float() {
                self.update(key, v.extract::<T>() + val);
            }
        } else {
            self.set(key, val);
        }
    }

    pub fn decr<S: AsRef<str>>(&mut self, key: S) {
        if self.has_key(key.as_ref()) {
            let v = self.get(key.as_ref()).clone();
            if v.is_int() || v.is_uint() || v.is_float() {
                match v.type_name().as_str() {
                    "i8" => self.update(key, v.extract::<i8>() - 1),
                    "i16" => self.update(key, v.extract::<i16>() - 1),
                    "i32" => self.update(key, v.extract::<i32>() - 1),
                    "i64" => self.update(key, v.extract::<i64>() - 1),
                    "i128" => self.update(key, v.extract::<i128>() - 1),
                    "u8" => self.update(key, v.extract::<u8>() - 1),
                    "u16" => self.update(key, v.extract::<u16>() - 1),
                    "u32" => self.update(key, v.extract::<u32>() - 1),
                    "u64" => self.update(key, v.extract::<u64>() - 1),
                    "u128" => self.update(key, v.extract::<u128>() - 1),
                    "f32" => self.update(key, v.extract::<f32>() - 1.0),
                    "f64" => self.update(key, v.extract::<f64>() - 1.0),
                    _ => panic!("Something went wrong"),
                }
            }
        } else {
            self.set(key, 1 as isize);
        }
    }

    pub fn decr_by<S, T>(&mut self, key: S, val: T)
    where
        S: AsRef<str>,
        T: serde::ser::Serialize + serde::de::DeserializeOwned + std::ops::Sub,
        <T as std::ops::Sub>::Output: serde::ser::Serialize,
    {
        if self.has_key(key.as_ref()) {
            let v = self.get(key.as_ref()).clone();
            if v.is_int() || v.is_uint() || v.is_float() {
                self.update(key, v.extract::<T>() - val);
            }
        } else {
            self.set(key, val);
        }
    }

    pub fn swap<S: AsRef<str>, T: serde::Serialize>(&mut self, key: S, value: T) -> BvObject {
        let new_obj = serialize_object(&value);
        let old_obj = self.records.get_mut(key.as_ref().as_bytes());

        if new_obj.type_name() == old_obj.type_name() && new_obj.raw().len() == old_obj.raw().len()
        {
            return std::mem::replace(old_obj, new_obj);
        }

        panic!("Not same type or equal length")
    }

    pub fn update<S: AsRef<str>, T: serde::Serialize>(&mut self, key: S, value: T) {
        // Update
        let old = self.records.get(key.as_ref().as_bytes()).type_name();
        let new = serialize_object(&value);

        if old.inner().len() == new.inner().len() {
            *self.records.get_mut(key.as_ref().as_bytes()) = new;
        } else {
            self.del(key.as_ref());
            self.records
                .insert(key.as_ref().as_bytes().to_vec().into(), new);
        }
    }

    pub fn set<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, key: S, value: T) {
        // Create new
        let value = serialize_object(&value);
        assert!(!key.as_ref().is_empty() && !value.type_name().is_empty());

        self.records
            .insert(key.as_ref().as_bytes().to_vec().into(), value);
    }

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

    pub fn get<S: AsRef<str>>(&self, key: S) -> &BvObject {
        &self.records.get(key.as_ref().as_bytes())
    }

    pub fn get_value<T: serde::de::DeserializeOwned>(&self, key: &str) -> T {
        self.records.get(key.as_bytes()).extract()
    }

    pub fn get_tuple<S: AsRef<str>>(&mut self, key: S) -> BvTuple {
        BvTuple::from(self.records.get_mut(key.as_ref().as_bytes()))
    }

    pub fn get_str<S: AsRef<str>>(&mut self, key: S) -> BvStr {
        BvStr::from_bvobject(self.records.get_mut(key.as_ref().as_bytes()))
    }

    pub fn del<S: AsRef<str>>(&mut self, key: S) -> BvObject {
        self.records.remove(key.as_ref().as_bytes())
    }
}

impl<KV> BytesFilter for KvDb<KV>
where
    KV: KvInterface<Key = BvString, Value = BvObject, RefKey = [u8]>,
    for<'a> &'a KV: IntoIterator<Item = &'a (BvString, BvObject)>,
{
    fn filter<F>(&self, cb: F) -> Vec<&(BvString, BvObject)>
    where
        F: Fn(&(BvString, BvObject)) -> bool,
    {
        (&self.records).into_iter().filter(|t| cb(t)).collect()
    }
}

impl<KV: KvInterface<Key = BvString, Value = BvObject, RefKey = [u8]>> BytesSearch for KvDb<KV>
where
    KV: KvInterface<Key = BvString, Value = BvObject, RefKey = [u8]>,
    for<'a> &'a KV: IntoIterator<Item = &'a (BvString, BvObject)>,
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
