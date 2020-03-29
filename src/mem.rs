use std::collections::HashMap;

use crate::decl::types::*;


/// Stores byte data of key, type_name and value
pub type OwnedMemoryRecord = (Vec<u8>, Vec<u8>, Vec<u8>);


pub struct Memory {
	/// KV storage
	kv_records: Vec<OwnedMemoryRecord>,
	/// Group declaration and storage
	decl_map: DeclarationMap,
	decl_records: DeclarationRecords,

	/// Lookup keys index in records vec
	lu_map_exact: HashMap<Vec<u8>, usize>,
	/// Lookup keys starting with u8 in records vec
	lu_map_first: HashMap<u8, Vec<usize>>,
}

impl Memory {
	pub fn new() -> Self {
		Memory {
			kv_records: Vec::new(),
			decl_map: DeclarationMap::new(),
			decl_records: DeclarationRecords::new(),
			lu_map_exact: HashMap::new(),
			lu_map_first: HashMap::new(),
		}
	}

	pub fn decl_get_mut(&mut self) -> &mut DeclarationMap {
		&mut self.decl_map
	}

	pub fn kv_records_get_mut(&mut self) -> &mut Vec<OwnedMemoryRecord> {
		&mut self.kv_records
	}

	pub fn decl_records_get_mut(&mut self) -> &mut DeclarationRecords {
		&mut self.decl_records
	}

	pub fn generate_lu_maps(&mut self) {
		let mut lu_map_exact = HashMap::with_capacity(self.kv_records.len());
		let mut lu_map_first = HashMap::with_capacity(self.kv_records.len());

		for (i, (k, _, _)) in self.kv_records.iter().enumerate() {
			lu_map_exact.insert(k.to_vec(), i);
			lu_map_first.entry(k[0])
				.and_modify(|ref mut v: &mut Vec<usize>| v.push(i))
				.or_insert(Vec::new());
		}

		self.lu_map_first = lu_map_first;
		self.lu_map_exact = lu_map_exact;
	}

	pub fn declaration_map(&self) -> &DeclarationMap {
		&self.decl_map
	}

	pub fn kv_records(&self) -> &Vec<OwnedMemoryRecord> {
		&self.kv_records
	}

	pub fn decl_records(&self) -> &DeclarationRecords {
		&self.decl_records
	}

	pub fn index_of_key(&self, key: &[u8]) -> usize {
		self.lu_map_exact[key]
	}

	pub fn char_search(&self, r#char: u8) -> Vec<&OwnedMemoryRecord> {
		self.lu_map_first[&r#char].iter().map(|i| &self.kv_records[*i]).collect()
	}

	pub fn push_record(&mut self, r: (Vec<u8>, Vec<u8>, Vec<u8>)) {
		if self.lu_map_exact.contains_key(&r.0) {
			self.delete_record(self.index_of_key(&r.0))
		}

		let new_idx = self.kv_records.len();
		self.lu_map_exact.insert(r.0.to_vec(), self.kv_records.len());
		self.lu_map_first.entry(r.0[0])
			.and_modify(|v| v.push(new_idx))
			.or_insert(Vec::new());
		self.kv_records.push(r);
	}

	pub fn delete_record(&mut self, i: usize) {
		let (k, _, _) = &self.kv_records.remove(i);
		self.lu_map_exact.remove(k);
		self.lu_map_first.get_mut(&k[0]).unwrap().remove_item(&i);
	}

	pub fn decls(&self) -> &DeclarationMap {
		&self.decl_map
	}

	pub fn insert_decl(&mut self, decl: &Declare) {
		self.decl_map.insert(decl.raw_name().to_vec(), decl.fields().to_owned());
		self.decl_records.insert(decl.raw_name().to_vec(), Vec::new());
	}

	pub fn remove_decl<S: AsRef<str>>(&mut self, key: S) {
		self.decl_map.remove(key.as_ref().as_bytes());
		self.decl_records.remove(key.as_ref().as_bytes());
	}

	pub fn decl_insert_row(&mut self, name: &str, record: DeclarationRecord) {
		let _field_rules = self.decl_map.get(name.as_bytes());
		
		self.decl_records.entry(name.as_bytes().to_vec())
			.and_modify(|v| v.push(record));
	}

	pub fn get_field_map<S: AsRef<str>>(&self, name: S) -> &FieldMap {
		&self.decl_map[name.as_ref().as_bytes()]
	}

	pub fn get_decl_records<S: AsRef<str>>(&self, name: S) -> &Vec<DeclarationRecord> {
		&self.decl_records.get(name.as_ref().as_bytes()).expect(&format!("No entry found for key {:?}", name.as_ref()))
	}
}

impl std::ops::Deref for Memory {
	type Target = Vec<OwnedMemoryRecord>;

	fn deref(&self) -> &Self::Target {
		&self.kv_records
	}
}

impl std::ops::DerefMut for Memory {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.kv_records
	}
}

impl<S: AsRef<str>> std::ops::Index<S> for Memory {
	type Output = OwnedMemoryRecord;

	fn index(&self, key: S) -> &Self::Output {
		if !self.lu_map_exact.contains_key(key.as_ref().as_bytes()) {
			panic!("No entry found for key: {:?}", key.as_ref())
		}

		&self.kv_records[self.lu_map_exact[key.as_ref().as_bytes()]]
	}
}

impl<S: AsRef<str>> std::ops::IndexMut<S> for Memory {
	fn index_mut(&mut self, key: S) -> &mut Self::Output {
		&mut self.kv_records[self.lu_map_exact[key.as_ref().as_bytes()]]
	}
}

