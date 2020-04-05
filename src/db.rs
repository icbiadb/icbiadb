use crate::utils::{serialize, serialize_object, normalize_type_name};
use crate::decl::types::*;
use crate::prelude::*;
use crate::mem::{Memory, MemState};
use crate::fio::FileIO;
use crate::types::bv::{BvStr, BvObject};
use crate::storage::IndexedKvStorage;
use crate::storage::KvInterface;

enum DbType {
	InMemory,
	File,
}

pub struct Db {
	file_name: String,
	f_io: Option<FileIO>,
	memory: Memory<IndexedKvStorage>,
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
		let mut memory = Memory::new(MemState::ReadWrite);
		f_io.read_to(&mut memory)?;
		
		
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
		let mut memory = Memory::new(MemState::ReadWrite);
		f_io.read_to(&mut memory)?;
		
		
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
		let mut memory = Memory::new(MemState::ReadWrite);
		f_io.read_to(&mut memory)?;
		

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
			memory: Memory::new(MemState::ReadWrite),
			r#type: DbType::InMemory,
		})
	}

	pub fn write_only<S: AsRef<str>>(file_name: S) -> std::io::Result<Self> {
		let f = std::fs::OpenOptions::new()
			.write(true)
			.create(true)
			.read(true)
			.open(file_name.as_ref())?;

		let f_io = FileIO::new(f);
		let mut memory = Memory::new(MemState::WriteOnly);
		f_io.read_to(&mut memory)?;
		
		Ok(Db {
			file_name: file_name.as_ref().to_string(),
			f_io: Some(f_io),
			memory: memory,
			r#type: DbType::File,
		})	
	}

	pub fn file_name(&self) -> &str {
		self.file_name.as_str()
	}

	pub fn memory(&self) -> &Memory<IndexedKvStorage> {
		&self.memory
	}

	pub fn memory_mut(&mut self) -> &mut Memory<IndexedKvStorage> {
		&mut self.memory
	}

	pub fn len(&self) -> usize {
		self.memory.len()
	}

	pub fn declarations(&self) -> &DeclarationMap {
		&self.memory.declaration_map()
	}

	pub fn kv_records(&self) -> &IndexedKvStorage {
		&self.memory.kv_records()
	}

	pub fn decl_records(&self) -> &DeclarationRecords {
		&self.memory.decl_records()
	}

	pub fn field_map<S: AsRef<str>>(&self, key: S) -> &FieldMap {
		&self.memory.declaration_map()[key.as_ref().as_bytes()]
	}

	pub fn has_key<S: AsRef<str>>(&self, key: S) -> bool {
		self.memory.contains_key(key.as_ref().as_bytes())
	}

	pub fn swap<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, key: S, new_val: T) -> BvObject {
		let new = serialize_object(&new_val);
		if self.memory[key.as_ref().as_bytes()].type_name() == new.type_name() {
			return std::mem::replace(&mut self.memory.get_mut(key.as_ref().as_bytes()), new)
		}

		panic!("Not equal type, key: {}", key.as_ref())
	}

	pub fn store<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, k: S, v: T) {
		let (k, v) = (k.as_ref().as_bytes(), serialize_object(&v));
		assert!(k.len() > 0 && v.type_name().len() > 0);
		self.memory.push_record((k.into(), v));
	}

	pub fn store_as<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, k: S, t: S, v: T) {
		let (k, v) = (
			k.as_ref().as_bytes(), 
			BvObject::from_raw(normalize_type_name(t.as_ref().as_bytes()).to_vec(), serialize(&v))
		);

		assert!(k.len() > 0 && v.type_name().len() > 0);
		self.memory.push_record((k.into(), v));
	}

	pub fn store_raw<S: AsRef<str>>(&mut self, k: S, t: S, v: Vec<u8>) {	
		let (k, v) = (k.as_ref().as_bytes(), BvObject::from_raw(normalize_type_name(t.as_ref().as_bytes()).to_vec(), v));
		assert!(k.len() > 0 && v.type_name().len() > 0);
		self.memory.push_record((k.into(), v));
	}

	pub fn store_many<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, values: &Vec<(S, T)>) {
		for (k, v) in values {
			let (k, v) = (k.as_ref().as_bytes(), serialize_object(&v));
			assert!(k.len() > 0 && v.type_name().len() > 0);
			self.memory.push_record((k.into(), v));
		}
	}

	pub fn store_many_as<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, values: &Vec<(S, S, T)>) {
		for (k, t, v) in values {
			let (k, v) = (
				k.as_ref().as_bytes(), 
				BvObject::from_raw(normalize_type_name(t.as_ref().as_bytes()).to_vec(), serialize(&v))
			);
			assert!(k.len() > 0 && v.type_name().len() > 0);
			self.memory.push_record((k.into(), v));
		}
	}

	pub fn fetch<S: AsRef<str>>(&self, key: S) -> &BvObject {
		&self.memory[key.as_ref().as_bytes()]
	}

	pub fn fetch_value<T: serde::de::DeserializeOwned>(&self, key: &str) -> T {
		self.memory[key.as_bytes()].extract()
	}

	pub fn update<S: AsRef<str>, T: serde::ser::Serialize>(&mut self, k: S, v: T) {
		let old = self.memory[k.as_ref().as_bytes()].type_name();
		let new = serialize_object(&v);
		if old == new.type_name() {
			*self.memory.get_mut(k.as_ref().as_bytes()) = new;
		} else {
			self.memory.delete_record(k.as_ref().as_bytes());
			self.store(k, v);
		}
	}

	pub fn remove<S: AsRef<str>>(&mut self, key: S) {
		self.memory.delete_record(key.as_ref().as_bytes());
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
			self.f_io.as_mut().unwrap().dump_mem::<IndexedKvStorage>(&self.memory).unwrap()
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
		self.memory.remove_decl(name.as_ref().as_bytes());
	}

	pub fn decl_get_field_map<S: AsRef<str>>(&self, name: S) -> &FieldMap {
		self.memory.get_field_map(name.as_ref().as_bytes())
	}

	pub fn get_decl_records<S: AsRef<str>>(&self, name: S) -> &Vec<DeclarationRecord> {
		self.memory.get_decl_records(name.as_ref().as_bytes())
	}

	pub fn decl_insert_row<S: AsRef<str>>(&mut self, name: S, row: DeclarationRecord) -> Result<(), String> {
		let field_map = self.memory.get_field_map(&name.as_ref().as_bytes());
		let records = self.memory.get_decl_records(name.as_ref().as_bytes());
		let mut unique_values = std::collections::HashMap::new();
		let unique_fields: Vec<Vec<u8>> = field_map.iter()
			.filter_map(|(k, v)| {
				if v.contains_key("unique".as_bytes()) {
					return Some(k.to_vec())
				} else { None }
			})
			.collect();

		for (k, v) in field_map.iter() {
			if v["type".as_bytes()] != row[k].type_name() {
				return Err(format!("Expected type \"{}\" found type \"{}\"", v["type".as_bytes()].as_str(), row[k].type_name()));
			}

			if v.contains_key("unique".as_bytes()) {
				unique_values.insert(k.to_vec(), &row[k]);
			}
		}

		for srow in records.iter() {
			for unique_field in unique_fields.iter() {
				if srow[unique_field.as_slice()] == unique_values[unique_field.as_slice()] {
					return Err(format!("\"{}\" collided", std::str::from_utf8(unique_field).unwrap()));
				}
			}
		}

		self.memory.decl_insert_row(name.as_ref().as_bytes().to_vec(), row);
		Ok(())
	}

	pub fn decl_insert_many<S: AsRef<str>>(&mut self, name: S, mut rows: Vec<DeclarationRecord>) -> Result<(), String> {
		for row in rows.drain(0..rows.len()) {
			self.decl_insert_row(&name, row)?
		}

		Ok(())
	}

	pub fn query<S: AsRef<str>>(&self, name: S) -> QueryBuilder {
		let field_map = self.memory.get_field_map(name.as_ref().as_bytes());
		let records = self.memory.get_decl_records(name.as_ref().as_bytes());
		QueryBuilder::new(field_map, records)
	}
}

impl BytesFilter for Db {
	fn filter<F>(&self, cb: F) -> Vec<(BvStr, &BvObject)> where F: Fn((BvStr, &BvObject)) -> bool {
		(&self.memory.kv_records()).into_iter()
			.filter_map(|(k, v)| {
				if cb((BvStr::from(k), v)) { return Some((BvStr::from(k), v)) } 
				None
			})
			.collect()
	}
}



impl BytesSearch for Db {
	fn starts_with<S: AsRef<str>>(&self, key_part: S) -> Vec<(BvStr, &BvObject)> {
		let k_part = key_part.as_ref().as_bytes();
		(&self.memory.kv_records()).into_iter()
			.filter_map(|(k, v)| {
				if k.starts_with(k_part) {
					return Some((BvStr::from(k), v))
				}

				None
			})
			.collect()
	}

	fn ends_with<S: AsRef<str>>(&self, key_part: S) -> Vec<(BvStr, &BvObject)> {
		(&self.memory.kv_records()).into_iter()
			.filter_map(|(k, v)| {
				let k = BvStr::from(k);
				if k.ends_with(key_part.as_ref().as_bytes()) {
					return Some((k, v))
				}

				None
			})
			.collect::<Vec<_>>()
	}

	fn contains<S: AsRef<str>>(&self, key_part: S) -> Vec<(BvStr, &BvObject)> {
		(&self.memory.kv_records()).into_iter()
			.filter_map(|(k, v)| {
				let k = BvStr::from(k);
				if k.contains(key_part.as_ref()) {
					return Some((k, v))
				}

				None
			})
			.collect::<Vec<_>>()
	}
}



