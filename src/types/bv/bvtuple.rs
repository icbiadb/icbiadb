use super::{BvObject, BvStr};
use crate::slice;
use crate::normalize_type_name;


#[derive(Debug)]
pub struct BvTuple<'a>{
	inner: &'a BvObject,
	element_index: Vec<usize>,
	type_map: Vec<BvStr<'a>>,
}

impl<'a> BvTuple<'a> {
	pub fn from(obj: &'a BvObject) -> Self {
		
		let type_map = slice::split(&obj.type_name()[1..obj.type_name().len()-1], b", ").iter()
			.map(|r| BvStr::new(normalize_type_name(r)))
			.collect::<Vec<_>>();

		let mut ele_index = vec![0];

		// TODO
		// Create Borrowed BvObject

		for r#type in type_map.iter() {
			let r = match r#type.as_slice() {
				[105, 51, 50] => { 4 }
				_ => { panic!("Failed to calculate tuple size") }
			};

			ele_index.push(r);
		}
		
		
		BvTuple {
			inner: obj,
			element_index: ele_index,
			type_map: type_map,
		}
	}

	pub fn update<T: Sized + serde::Serialize>(&mut self, _indx: usize, _value: T) {

	}
}






