use super::{BvObject, BvStr, BvInt};
use crate::slice;
use crate::normalize_type_name;
use crate::utils::{deserialize, is_int};

#[derive(Debug)]
pub struct BvTuple<'a>{
	inner: &'a BvObject,
	indexes: Vec<usize>,
	type_map: Vec<BvStr<'a>>,
}

impl<'a> BvTuple<'a> {
	pub fn from(obj: &'a BvObject) -> Self {
		
		let type_map = slice::split(&obj.type_name()[1..obj.type_name().len()-1], b", ").iter()
			.map(|r| BvStr::new(normalize_type_name(r)))
			.collect::<Vec<_>>();


		// TODO
		// Create Borrowed BvObject

		let mut ele_index = vec![];
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
				[115, 116, 114] => 8, // Length of str
				_ => { panic!("Failed to calculate tuple size") }
			};

			ele_index.push(r);
		}
		
		let mut start = 0;
		for (i, e) in type_map.iter().enumerate() {
			if e == "str" {
				ele_index[i] += deserialize::<usize>(&obj[start..start+8]);
			}
			start += ele_index[i];
		}
		
		BvTuple {
			inner: obj,
			indexes: ele_index,
			type_map: type_map,
		}
	}

	pub fn get_start(&self, indx: usize) -> usize {
		let mut start = 0;
		for i in 0..indx {
			start += self.indexes[i];
		}

		start
	}

	pub fn get(&self, indx: usize) -> BvStr {
		let r = &self.inner[self.get_start(indx)..self.get_start(indx)+self.indexes[indx]];

		if self.type_map[indx] == "str" {
			BvStr::new(&r[8..])
		} else if is_int(self.type_map[indx].as_slice()) {
			BvStr::new(b"int")
		} else {
			panic!("Failed to implicit cast byte slice to type")
		}
	}

	pub fn update<T: Sized + serde::Serialize>(&mut self, _indx: usize, _value: T) {
	}
}






