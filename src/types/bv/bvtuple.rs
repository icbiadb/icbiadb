use super::{BvObj, BvObject, BvString};
use crate::normalize_type_name;
use crate::slice;
use crate::utils::{deserialize, is_int, serialize_object};

#[derive(Debug)]
pub struct BvTuple<'a> {
    inner: &'a mut BvObject,
    elength: Vec<usize>,
    type_map: Vec<BvString>,
}

impl<'a> BvTuple<'a> {
    pub fn from(obj: &'a mut BvObject) -> Self {
        // TODO
        // Optimize away, calculate element len and JIT type name check instead of vec alloc
        let type_map = slice::split(&obj.type_name()[1..obj.type_name().len() - 1], b", ")
            .iter()
            .map(|r| BvString::from(normalize_type_name(r).to_vec()))
            .collect::<Vec<_>>();

        let mut elength = vec![];
        for r#type in type_map.iter() {
            let r = match r#type.as_slice() {
                // i8-i128
                [105, 56] => 1,
                [105, 49, 54] => 2,
                [105, 51, 50] => 4,
                [105, 54, 52] => 8,
                [105, 49, 50, 56] => 16,
                // u8-u128
                [117, 56] => 1,
                [117, 49, 54] => 2,
                [117, 51, 50] => 4,
                [117, 54, 52] => 8,
                [117, 49, 50, 56] => 16,
                // f32-f64
                [102, 51, 50] => 4,
                [102, 54, 52] => 8,
                // Str
                [115, 116, 114] => {
                    // Length of str serialized by bincode, usize(str lengt) + str length
                    let curr_pos = elength.iter().sum();
                    8 + deserialize::<usize>(&obj[curr_pos..curr_pos + 8])
                }
                _ => panic!("Failed to calculate tuple size"),
            };

            elength.push(r);
        }

        BvTuple {
            inner: obj,
            elength,
            type_map,
        }
    }

    pub fn get_start(&self, index: usize) -> usize {
        let mut start = 0;
        for i in 0..index {
            start += self.elength[i];
        }

        start
    }

    pub fn get(&'a self, index: usize) -> BvObj<'a> {
        let r = &self.inner[self.get_start(index)..self.get_start(index) + self.elength[index]];

        if self.type_map[index] == "str" || is_int(self.type_map[index].as_slice()) {
            BvObj::new(self.type_map[index].as_slice(), r)
        } else {
            panic!("Failed to implicit cast byte slice to type")
        }
    }

    pub fn value<T: serde::de::DeserializeOwned>(&self, index: usize) -> T {
        let start = self.get_start(index);
        deserialize(&self.inner[start..start + self.elength[index]])
    }

    pub fn set<T: Sized + serde::Serialize>(&mut self, index: usize, value: T) {
        let new_value = serialize_object(&value);
        if self.type_map[index] != new_value.type_name() {
            panic!("Not the same type")
        }

        let length = self.elength[index];
        if length != new_value.len() {
            panic!(
                "Not the same length, overwriting len {} with len {}",
                length,
                new_value.len()
            )
        }

        let start = self.get_start(index);
        let new_slice = new_value.as_slice();

        for (x, i) in (start..start + length).enumerate() {
            self.inner[i] = new_slice[x];
        }
    }
}
