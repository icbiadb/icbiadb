use std::collections::HashMap;

use crate::decl::types::*;
use crate::types::bv::{BvString, BvObject};


#[derive(PartialEq)]
pub enum MemState {
	WriteOnly,
	ReadWrite
}

pub struct Memory {
	state: MemState,
	/// KV storage
	kv_records: HashMap<Vec<u8>, BvObject>,
	/// Group declaration and storage
	decl_map: DeclarationMap,
	decl_records: DeclarationRecords,
}

impl Memory {
	pub fn new(state: MemState) -> Self {
		Memory {
			state: state,
			kv_records: HashMap::new(),
			decl_map: DeclarationMap::new(),
			decl_records: DeclarationRecords::new(),
		}
	}

	pub fn decl_get_mut(&mut self) -> &mut DeclarationMap {
		&mut self.decl_map
	}

	pub fn kv_records_get_mut(&mut self) -> &mut HashMap<Vec<u8>, BvObject> {
		&mut self.kv_records
	}

	pub fn decl_records_get_mut(&mut self) -> &mut DeclarationRecords {
		&mut self.decl_records
	}

	pub fn declaration_map(&self) -> &DeclarationMap {
		&self.decl_map
	}

	pub fn kv_records(&self) -> &HashMap<Vec<u8>, BvObject> {
		&self.kv_records
	}

	pub fn decl_records(&self) -> &DeclarationRecords {
		&self.decl_records
	}

	pub fn push_record(&mut self, r: (BvString, BvObject)) {
		if MemState::WriteOnly == self.state {
			self.kv_records.insert(r.0.to_vec(), r.1);
			return
		}

		/*
		if self.kv_records.contains_key(r.0.as_slice()) {
			self.delete_record(self.index_of_key(r.0.as_slice()))
		}
		*/

		self.kv_records.insert(r.0.to_vec(), r.1);
	}

	pub fn delete_record(&mut self, key: &[u8]) {
		let _r = &self.kv_records.remove(key);
	}

	pub fn decls(&self) -> &DeclarationMap {
		&self.decl_map
	}

	pub fn insert_decl(&mut self, decl: &Declare) {
		self.decl_map.insert(decl.raw_name().to_vec(), decl.fields().to_owned());
		self.decl_records.insert(decl.raw_name().to_vec(), Vec::new());
	}

	pub fn remove_decl(&mut self, key: &[u8]) {
		self.decl_map.remove(key);
		self.decl_records.remove(key);
	}

	pub fn decl_insert_row(&mut self, name: Vec<u8>, record: DeclarationRecord) {
		let _field_rules = self.decl_map.get(name.as_slice());
		
		self.decl_records.entry(name)
			.and_modify(|v| v.push(record));
	}

	pub fn decl_insert_rows(&mut self, name: Vec<u8>, rows: Vec<DeclarationRecord>) {
		self.decl_records.entry(name)
			.and_modify(|v| v.extend(rows));
	}

	pub fn get_field_map(&self, name: &[u8]) -> &FieldMap {
		&self.decl_map[name]
	}

	pub fn get_decl_records(&self, name: &[u8]) -> &Vec<DeclarationRecord> {
		&self.decl_records.get(name).expect(&format!("No entry found for key {:?}", name.as_ref()))
	}
}

impl std::ops::Deref for Memory {
	type Target = std::collections::HashMap<Vec<u8>, BvObject>;

	fn deref(&self) -> &Self::Target {
		&self.kv_records
	}
}

impl std::ops::DerefMut for Memory {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.kv_records
	}
}


