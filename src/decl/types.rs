use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::utils::{serialize, serialize_to_bytevec, normalize_type_name};
use crate::types::bv::ByteVec;
use crate::db::Db;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldMap(HashMap<Vec<u8>, HashMap<Vec<u8>, ByteVec>>);

impl FieldMap {
	pub fn new() -> Self {
		FieldMap(HashMap::new())
	}
}

impl std::ops::Deref<> for FieldMap {
	type Target = HashMap<Vec<u8>, HashMap<Vec<u8>, ByteVec>>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for FieldMap {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl std::fmt::Display for FieldMap {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(fmt, "FieldMap {:?}", self.0.iter()
			.map(|(k, v)| {
				(k,
				v.iter()
					.map(|(k ,v)| (k, std::str::from_utf8(v.inner()).unwrap()))
					.collect::<Vec<_>>()
				)
			})
			.collect::<Vec<_>>()
		)
	}
}

/// Stores group name and serialized HashMap of fields + type
///
/// Used by mem::Memory
///
/// HashMap<Name, data>
///
/// name: Name of group
/// data: Map of field and field rules
pub type DeclarationMap = HashMap<Vec<u8>, FieldMap>;


/// Stores field names and field values
///
/// HashMap<Field Name, Field data>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclarationRecord(HashMap<Vec<u8>, ByteVec>);

impl DeclarationRecord {
	pub fn new() -> Self {
		DeclarationRecord(HashMap::new())
	}

	pub fn from_vec<S: AsRef<str>, T: Sized + serde::ser::Serialize>(v: Vec<(Vec<u8>, ByteVec)>) -> Self {
		DeclarationRecord::from_hashmap(v.iter().cloned()
			.filter_map(|(field, value)| {
				Some((field, value))
			})
			.collect::<HashMap<_, _>>()
		)
	}

	pub fn from_hashmap(hm: HashMap<Vec<u8>, ByteVec>) -> Self {
		DeclarationRecord(hm)
	}

	pub fn with_capacity(cap: usize) -> Self {
		DeclarationRecord(HashMap::with_capacity(cap))
	}

	pub fn insert<S: AsRef<str>, T: Sized + serde::ser::Serialize>(&mut self, field: S, value: T) {
		self.0.insert(field.as_ref().as_bytes().to_vec(), serialize_to_bytevec(&value));
	}
}

impl std::ops::Deref for DeclarationRecord {
	type Target = HashMap<Vec<u8>, ByteVec>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for DeclarationRecord {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl std::cmp::PartialEq for DeclarationRecord {
	fn eq(&self, other: &DeclarationRecord) -> bool {
		self["__rowid".as_bytes()] == other["__rowid".as_bytes()]
	}
}

/// Stores the records of a group declaration
///
/// Used by mem::Memory
pub type DeclarationRecords = HashMap<Vec<u8>, Vec<DeclarationRecord>>;


/// Stores the declaration of a group or a record
///
/// Used for db.declare("")...
pub struct Declare {
	name: Vec<u8>,
	fields: FieldMap,
	current_field: Vec<u8>
}

impl Declare {
	pub fn new<S: AsRef<str>>(name: S) -> Self {
		Declare {
			name: name.as_ref().as_bytes().to_vec(),
			fields: FieldMap::new(),
			current_field: Vec::new(),
		}
	}

	pub fn add_field<T: ?Sized + serde::ser::Serialize>(&mut self, name: &str) -> &mut Self {
		let type_name = normalize_type_name(std::any::type_name::<T>().as_bytes());

		let mut hm = HashMap::new();
		hm.insert("type".as_bytes().to_vec(), type_name.into());
		self.current_field = name.as_bytes().to_vec();
		self.fields.insert(name.as_bytes().to_vec(), hm);
		self
	}

	pub fn option<S: AsRef<str>, T: serde::ser::Serialize>(&mut self, k: S, v: T) -> &mut Self {
		self.fields.get_mut(&self.current_field).unwrap().entry(k.as_ref().as_bytes().to_vec())
			.and_modify(|r| *r = serialize(&v).into())
			.or_insert(serialize(&v).into());
		self
	}

	pub fn name(&self) -> &Vec<u8> {
		&self.name
	}

	pub fn raw_name(&self) -> &[u8] {
		&self.name
	}

	pub fn fields(&self) -> &FieldMap {
		&self.fields
	}

	pub fn data(&self) -> ByteVec {
		serialize_to_bytevec(&self.fields)
	}

	pub fn insert(&mut self, db: &mut Db) {
		db.insert_decl(self);
	}
}


// Used by macro query!
pub struct QueryResult<'a, T> {
	field_map: &'a FieldMap,
	records: Vec<T>,
}

impl<'a, T> QueryResult<'a, T> {
	pub fn new(field_map: &'a FieldMap, records: Vec<T>) -> Self {
		QueryResult {
			field_map: field_map,
			records: records
		}
	}

	pub fn field_map(&self) -> &'a FieldMap {
		self.field_map
	}
}

impl<'a, T> std::ops::Deref for QueryResult<'a, T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.records
	}
}

pub struct QueryBuilder<'a> {
	field_map: &'a FieldMap,
	records: &'a Vec<DeclarationRecord>,
	select_fields: Vec<&'a str>,
	filter: Option<Box<Fn(&HashMap<&str, &ByteVec>) -> bool>>
}

impl<'a> QueryBuilder<'a> {
	pub fn new(field_map: &'a FieldMap, records: &'a Vec<DeclarationRecord>) -> Self {
		QueryBuilder {
			field_map: field_map,
			records: records,
			select_fields: Vec::new(),
			filter: None,
		}
	}

	pub fn field_map(&self) -> &'a FieldMap {
		&self.field_map
	}

	pub fn records(&self) -> &'a Vec<DeclarationRecord> {
		&self.records
	}

	pub fn select(&mut self, fields: Vec<&'static str>) -> &mut Self {
		self.select_fields = fields;
		self
	}

	pub fn filter<F>(& mut self, cb: F) -> &mut Self where F: 'static + Fn(&HashMap<&str, &ByteVec>) -> bool {
		self.filter = Some(Box::new(cb));
		self
	}

	pub fn collect(&self) -> Vec<HashMap<&str, &ByteVec>> {
		self.records.iter().filter_map(|r| {
			let select_fields = self.select_fields.iter().cloned()
				.filter_map(|field| { Some((field, &r[field.as_bytes()])) })
				.collect::<HashMap<_, _>>();

			if self.filter.is_some() {
				if self.filter.as_ref().unwrap()(&select_fields) {
					return Some(select_fields)
				}

				return None
			} else {
				return Some(select_fields)
			}
		})
		.collect()
	}
}


