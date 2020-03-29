use std::collections::HashMap;

use crate::parser::globals::*;
use crate::utils::{serialize};
use crate::decl::types::*;
use crate::mem::Memory;

use std::io::{SeekFrom};

pub struct Writer<T: std::io::Write + std::io::Seek> {
	writer: T,
	declaration_length: u32,
	kv_records_length: u64,
	decl_records_length: u64,
	decl_lu_map: HashMap<Vec<u8>, u64>,
}

impl<T: std::io::Write + std::io::Seek> Writer<T> {
	pub fn new(writer: T) -> Self {
		Writer {
			writer: writer,
			declaration_length: 0,
			kv_records_length: 0,
			decl_records_length: 0,
			decl_lu_map: HashMap::new(),
		}
	}

	pub fn dump_memory(&mut self, memory: &Memory) -> std::io::Result<()> {
		self.write_header()?;

		// Declarations
		for (name, fields) in memory.decls() {
			self.declaration_length += self.write_declaration(name, fields)?;
		}
		self.writer.flush()?;

		// KV
		// TODO
		// For some reason u128 increased write size when casting from u64
		for record in memory.kv_records().iter() {
			self.kv_records_length += self.write_kv_record(record)?;
		}
		self.writer.flush()?;

		// Decl records
		for (name, records) in memory.decl_records() {
			let mut decl_lu_name = Vec::with_capacity(name.len() + "decl_records_start".as_bytes().len());
			decl_lu_name.extend(name);
			decl_lu_name.extend("decl_records_start".as_bytes());
			self.decl_lu_map.insert(decl_lu_name, self.writer.stream_position()?);

			let mut decl_records_length = 0;
			decl_records_length += self.write_decl_header()?;
			for record in records {
				decl_records_length += self.write_decl_record(record)?;
			}

			self.decl_records_length += decl_records_length;
			self.write_declaration_records_data(name, decl_records_length, records.len() as u64)?;
			self.writer.seek(SeekFrom::End(0))?;
		}

		self.write_header()?;
		self.writer.flush()?;

		Ok(())
	}

	pub fn write_header(&mut self) -> std::io::Result<()> {
		self.writer.seek(SeekFrom::Start(0))?;
		self.writer.write(&serialize(&self.declaration_length))?;
		self.writer.write(&serialize(&self.kv_records_length))?;
		self.writer.write(&serialize(&self.decl_records_length))?;
		Ok(())
	}

	pub fn write_declaration(&mut self, name: &Vec<u8>, fields: &FieldMap) -> std::io::Result<u32> {
		// Identifier, name length, fields length, name, fields
		let ser_fields = serialize(fields);
		let mut length = 0;

		length += self.writer.write(&decl::IDENT)? as u32;

		length += self.writer.write(&[name.len() as u8])? as u32;
		length += self.writer.write(&serialize(&(ser_fields.len() as u16)))? as u32;

		length += self.writer.write(&name)? as u32;
		length += self.writer.write(&ser_fields)? as u32;
		
		let mut decl_lu_name = Vec::with_capacity(name.len() + "header_rdata_start".as_bytes().len());
		decl_lu_name.extend(name);
		decl_lu_name.extend("header_rdata_start".as_bytes());

		self.decl_lu_map.insert(decl_lu_name, self.writer.stream_position()?);
		length += self.writer.write(&serialize(&(0 as u64)))? as u32; // Records start
		length += self.writer.write(&serialize(&(0 as u64)))? as u32; // Total records length
		length += self.writer.write(&serialize(&(0 as u64)))? as u32; // Records count

		Ok(length)
	}

	pub fn write_declaration_records_data(&mut self, name: &Vec<u8>, total_len: u64, records_count: u64) -> std::io::Result<()> {
		let mut decl_header_rdata_start = Vec::with_capacity(name.len() + "header_rdata_start".as_bytes().len());
		decl_header_rdata_start.extend(name);
		decl_header_rdata_start.extend("header_rdata_start".as_bytes());

		let mut decl_records_start = Vec::with_capacity(name.len() + "decl_records_start".as_bytes().len());
		decl_records_start.extend(name);
		decl_records_start.extend("decl_records_start".as_bytes());
		
		self.writer.seek(SeekFrom::Start(self.decl_lu_map[&decl_header_rdata_start]))?;
		self.writer.write(&serialize(&self.decl_lu_map[&decl_records_start]))?;
		self.writer.write(&serialize(&total_len))?;
		self.writer.write(&serialize(&records_count))?;

		Ok(())
	}

	pub fn write_kv_record(&mut self, record: &(Vec<u8>, Vec<u8>, Vec<u8>)) -> std::io::Result<u64> {
		// Identifier, name length, fields length, name, fields
		let (k, tn, v) = record;
		assert!(k.len() as u8 > 0 && tn.len() as u8 > 0);
		let mut length = 0;

		length += self.writer.write(&kv::IDENT)? as u64;

		// K, TN, V lengths
		length += self.writer.write(&[k.len() as u8])? as u64;
		length += self.writer.write(&[tn.len() as u8])? as u64;
		length += self.writer.write(&serialize(&(v.len() as u32)))? as u64;

		// Values
		length += self.writer.write(&k)? as u64;
		length += self.writer.write(&tn)? as u64;
		length += self.writer.write(&v)? as u64;
		Ok(length)
	}

	pub fn write_decl_header(&mut self) -> std::io::Result<u64> {
		let mut length = 0;
		length += self.writer.write(&decl::records::IDENT)? as u64;
		Ok(length)
	}

	pub fn write_decl_record(&mut self, hm: &DeclarationRecord) -> std::io::Result<u64> {
		let ser_hm = serialize(hm);
		let mut length = 0;

		length += self.writer.write(&decl::records::RECORD_IDENT)? as u64;
		length += self.writer.write(&serialize(&(ser_hm.len() as u32)))? as u64;
		length += self.writer.write(&ser_hm)? as u64;

		Ok(length)
	}
}


