use crate::decl::types::*;
use crate::types::bv::{BvObject};
use crate::storage::KvInterface;



pub struct Memory<KV: KvInterface<Key=Vec<u8>, Value=BvObject, RefKey=[u8]>> {
	/// KV storage
	kv_records: KV,
	/// Group declaration and storage
	decl_map: DeclarationMap,
	decl_records: DeclarationRecords,
}

impl<KV: KvInterface<Key=Vec<u8>, Value=BvObject, RefKey=[u8]>> Memory<KV> {
	pub fn new() -> Self {
		Memory {
			kv_records: KV::default(),
			decl_map: DeclarationMap::new(),
			decl_records: DeclarationRecords::new(),
		}
	}

	pub fn get(&self, key: &[u8]) -> &BvObject {
		self.kv_records.get(key)
	}

	pub fn get_mut(&mut self, key: &[u8]) -> &mut BvObject {
		self.kv_records.get_mut(key)
	}

	pub fn len(&self) -> usize {
		self.kv_records.len()
	}

	pub fn has_key(&self, key: &[u8]) -> bool {
		self.kv_records.has_key(key)
	}

	pub fn decl_get_mut(&mut self) -> &mut DeclarationMap {
		&mut self.decl_map
	}

	pub fn kv_records_get_mut(&mut self) -> &mut KV {
		&mut self.kv_records
	}

	pub fn decl_records_get_mut(&mut self) -> &mut DeclarationRecords {
		&mut self.decl_records
	}

	pub fn declaration_map(&self) -> &DeclarationMap {
		&self.decl_map
	}

	pub fn kv_records(&self) -> &KV {
		&self.kv_records
	}

	pub fn decl_records(&self) -> &DeclarationRecords {
		&self.decl_records
	}

	pub fn push_record(&mut self, r: (Vec<u8>, BvObject)) {
		self.kv_records.insert(r.0, r.1);
	}

	pub fn delete_record(&mut self, key: &[u8]) -> BvObject {
		self.kv_records.remove(key)
	}

	/*
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

	pub fn get_decl_records_mut(&mut self, name: &[u8]) -> &mut Vec<DeclarationRecord> {
		self.decl_records.get_mut(name).expect(&format!("No entry found for key {:?}", name.as_ref()))
	}
	*/
}


impl<KV: KvInterface<Key=Vec<u8>, Value=BvObject, RefKey=[u8]>> std::ops::Index<&[u8]> for Memory<KV> {
	type Output = BvObject;

	fn index(&self, index: &[u8]) -> &Self::Output {
		self.kv_records.get(index)
	}
}

impl<KV: KvInterface<Key=Vec<u8>, Value=BvObject, RefKey=[u8]>> std::ops::IndexMut<&[u8]> for Memory<KV> {
	fn index_mut(&mut self, index: &[u8]) -> &mut Self::Output {
		self.kv_records.get_mut(index)
	}
}
