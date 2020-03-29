use crate::utils::{serialize, deserialize, normalize_type_name};
use crate::decl::types::*;
use crate::types::*;
use crate::prelude::*;
use crate::mem::{Memory, OwnedMemoryRecord};
use crate::fio::FileIO;
use crate::slice;

enum DbType {
	InMemory,
	File,
}

pub struct Db {
	file_name: String,
	f_io: Option<FileIO>,
	memory: Memory,
	r#type: DbType,
}

impl Db {
	pub fn create<S: AsRef<str>>(file_name: S) -> std::io::Result<Self> {
		let f = std::fs::OpenOptions::new()
			.write(true)
			.create(true)
			.read(true)
			.open(file_name.as_ref())?;

		let f_io = FileIO::new(f);
		let mut memory = Memory::new();
		f_io.read_to(&mut memory)?;
		memory.generate_lu_maps();
		
		Ok(Db {
			file_name: file_name.as_ref().to_string(),
			f_io: Some(f_io),
			memory: memory,
			r#type: DbType::File,
		})
	}

	pub fn read<S: AsRef<str>>(file_name: S) -> std::io::Result<Self> {
		let f = std::fs::OpenOptions::new()
			.write(true)
			.read(true)
			.open(file_name.as_ref())?;

		let f_io = FileIO::new(f);
		let mut memory = Memory::new();
		f_io.read_to(&mut memory)?;
		memory.generate_lu_maps();
		
		Ok(Db {
			file_name: file_name.as_ref().to_string(),
			f_io: Some(f_io),
			memory: memory,
			r#type: DbType::File,
		})
	}

	pub fn read_to_mem<S: AsRef<str>>(file_name: S) -> std::io::Result<Self> {
		let f = std::fs::OpenOptions::new()
			.read(true)
			.open(file_name.as_ref())?;


		let f_io = FileIO::new(f);
		let mut memory = Memory::new();
		f_io.read_to(&mut memory)?;
		memory.generate_lu_maps();

		Ok(Db {
			file_name: file_name.as_ref().to_string(),
			f_io: Some(f_io),
			memory: memory,
			r#type: DbType::InMemory,
		})
	}

	pub fn mem() -> std::io::Result<Self> {
		Ok(Db {
			file_name: String::new(),
			f_io: None,
			memory: Memory::new(),
			r#type: DbType::InMemory,
		})
	}

	pub fn file_name(&self) -> &str {
		self.file_name.as_str()
	}

	pub fn memory(&self) -> &Memory {
		&self.memory
	}

	pub fn memory_mut(&mut self) -> &mut Memory {
		&mut self.memory
	}

	pub fn len(&self) -> usize {
		self.memory.len()
	}

	pub fn declarations(&self) -> &DeclarationMap {
		&self.memory.declaration_map()
	}

	pub fn kv_records(&self) -> &Vec<OwnedMemoryRecord> {
		&self.memory.kv_records()
	}

	pub fn decl_records(&self) -> &DeclarationRecords {
		&self.memory.decl_records()
	}

	pub fn field_map<S: AsRef<str>>(&self, key: S) -> &FieldMap {
		&self.memory.declaration_map()[key.as_ref().as_bytes()]
	}

	pub fn has_key<S: AsRef<str>>(&self, key: S) -> bool {
		self.memory.has_key(key.as_ref().as_bytes())
	}

	pub fn store<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, k: S, v: T) {
		let (k, t, ser_v) = (k.as_ref().as_bytes(), normalize_type_name(std::any::type_name::<T>().as_bytes()), serialize(&v));
		assert!(k.len() > 0 && t.len() > 0);
		self.memory.push_record((k.to_vec(), t.to_vec(), ser_v));
	}

	pub fn store_as<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, k: S, t: S, v: T) {
		let (k, t, v) = (k.as_ref().as_bytes(), normalize_type_name(t.as_ref().as_bytes()), serialize(&v));
		assert!(k.len() > 0 && t.len() > 0);
		self.memory.push_record((k.to_vec(), t.to_vec(), v.to_vec()));
	}

	pub fn store_raw<S: AsRef<str>>(&mut self, k: S, t: S, v: &[u8]) {	
		let (k, t, v) = (k.as_ref().as_bytes(), normalize_type_name(t.as_ref().as_bytes()), v);
		assert!(k.len() > 0 && t.len() > 0);
		self.memory.push_record((k.to_vec(), t.to_vec(), v.to_vec()));
	}

	pub fn store_many<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, values: &Vec<(S, T)>) {
		for (k, v) in values {
			let (k, t, v) = (k.as_ref().as_bytes(), std::any::type_name::<T>().as_bytes(), &serialize(v));
			assert!(k.len() > 0 && t.len() > 0);
			self.memory.push_record((k.to_vec(), slice::strip_ref_symbols(t).to_vec(), v.to_vec()));
		}
	}

	pub fn store_many_as<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, values: &Vec<(S, S, T)>) {
		for (k, t, v) in values {
			let (k, t, v) = (k.as_ref().as_bytes(), t.as_ref().as_bytes(), &serialize(&v));
			assert!(k.len() > 0 && t.len() > 0);
			self.memory.push_record((k.to_vec(), t.to_vec(), v.to_vec()));
		}
	}

	pub fn fetch<S: AsRef<str>>(&self, key: S) -> Record {
		let (k, t, v) = &self.memory[key];
		Record::new(k, t, v)
	}

	pub fn fetch_value<T: serde::de::DeserializeOwned>(&self, key: &str) -> T {
		deserialize(&self.memory[key].2)
	}

	pub fn fetch_raw<S: AsRef<str>>(&self, key: S) -> &OwnedMemoryRecord {
		&self.memory[key]
	}

	pub fn fetch_mut<S: AsRef<str>>(&mut self, key: S) -> OwnedRecord {
		let (k, t, v) = &self.memory[key];
		OwnedRecord::new(k.to_vec(), t.to_vec(), v.to_vec())
	}

	pub fn update<S: AsRef<str>, T: serde::ser::Serialize>(&mut self, k: S, v: T) {
		self.memory.delete_record(self.memory.index_of_key(k.as_ref().as_bytes()));
		self.store(k, v);
	}

	pub fn update_record(&mut self, record: impl RecordRead) {
		self.memory.delete_record(self.memory.index_of_key(record.raw_key()));
		self.memory.push_record(record.to_tuple());
	}

	pub fn update_many(&mut self, records: &Vec<impl RecordRead>) {
		for record in records {
			self.memory.delete_record(self.memory.index_of_key(record.raw_key()));
			self.memory.push_record(record.to_tuple());
		}	
	}

	pub fn remove<S: AsRef<str>>(&mut self, key: S) {
		self.memory.delete_record(self.memory.index_of_key(key.as_ref().as_bytes()));
	}

	pub fn remove_record(&mut self, record: impl RecordRead) {
		self.memory.delete_record(self.memory.index_of_key(record.raw_key()));
	}

	pub fn remove_many(&mut self, records: &Vec<impl RecordRead>) {
		for record in records {
			self.memory.delete_record(self.memory.index_of_key(record.raw_key()));		
		}
	}

	pub fn commit(&mut self) -> std::io::Result<()> {
		if !self.file_name.is_empty() {
			let f = std::fs::OpenOptions::new()
				.write(true)
				.read(true)
				.create(true)
				.truncate(true)
				.open(&self.file_name)?;

			self.f_io = Some(FileIO::new(f));
			self.r#type = DbType::File;
			self.f_io.as_mut().unwrap().dump_mem(&self.memory).unwrap()
		}

		Err(std::io::Error::new(std::io::ErrorKind::Other, "File name not set, are you using a memory database?"))
	}

	pub fn has_decl<S: AsRef<str>>(&self, name: S) -> bool {
		self.memory.declaration_map().contains_key(name.as_ref().as_bytes())
	}

	pub fn declare<'a, S: AsRef<str>>(&mut self, name: S) -> Declare {
		Declare::new(name)
	}

	pub fn insert_decl(&mut self, dg: &Declare) {
		self.memory.insert_decl(dg);
	}

	pub fn remove_decl<S: AsRef<str>>(&mut self, name: S) {
		self.memory.remove_decl(name);
	}

	pub fn query<S: AsRef<str>>(&self, name: S) -> QueryBuilder {
		let field_map = self.memory.get_field_map(&name);
		let records = self.memory.get_decl_records(&name);
		QueryBuilder::new(field_map, records)
	}
}

impl BytesFilter for Db {
	fn filter<F>(&self, cb: F) -> Vec<Record> where F: Fn(&Record) -> bool {
		self.memory.iter()
			.filter_map(|r| {
				let r: Record = r.into();
				if cb(&r) { return Some(r) } 
				None
			})
			.collect()
	}
}

impl BytesSearch for Db {
	fn starts_with<S: AsRef<str>>(&self, key_part: S) -> Vec<Record> {
		let k_part = key_part.as_ref().as_bytes();
		self.memory.char_search(k_part[0]).iter().filter_map(|(k, t, v)| {
			if k_part.len() > k.len() {
				return None
			}

			
			if k.starts_with(k_part) {
				return Some(Record::new(k, t, v))
			}

			None
		})
		.collect::<Vec<_>>()
	}

	fn ends_with<S: AsRef<str>>(&self, key_part: S) -> Vec<Record> {
		let k_part = key_part.as_ref().as_bytes();
		self.memory.iter().filter_map(|(k, t, v)| {
			if k_part.len() > k.len() {
				return None
			}

			if k.ends_with(k_part) {
				return Some(Record::new(k, t, v))
			}

			None
		})
		.collect::<Vec<_>>()
	}

	fn contains<S: AsRef<str>>(&self, key_part: S) -> Vec<Record> {
		let k_part = key_part.as_ref().as_bytes();
		self.memory.iter().filter_map(|(k, t, v)| {
			if k_part.len() > k.len() {
				return None
			}

			let mut k_part_idx = 0;
			for b in k.iter() {
				if k_part_idx == k_part.len() {
					return Some(Record::new(k, t, v))
				}

				if k_part[k_part_idx] == *b {
					k_part_idx += 1;
				} else {
					k_part_idx = 0;
				}

			}

			None
		})
		.collect::<Vec<_>>()
	}
}



