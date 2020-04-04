use std::collections::HashMap;

use serde::Deserialize;

use crate::utils::*;
use crate::decl::types::*;
use crate::mem::{Memory};
use crate::parser;
use crate::types::BvObject;

use std::io::{SeekFrom};

#[derive(Debug, Deserialize)]
#[repr(C)]
pub struct Header {
	decl_len: u32,
	records_len: u64,
	decl_records_len: u64,
}

pub struct Reader<T: std::io::BufRead + std::io::Seek> {
	reader: T,
}

impl<T: std::io::BufRead + std::io::Seek> Reader<T> {
	pub fn new(reader: T) -> Self {
		Reader {
			reader: reader,
		}
	}

	pub fn read_to(&mut self, memory: &mut Memory) -> std::io::Result<()> {
		// TODO, move data initialization for memory upward in the function call stack
		#[cfg(test)]
		let time = std::time::Instant::now();

		if let Ok(buf) = self.reader.fill_buf() {
			// Empty file
			if buf.len() == 0 {
				return Ok(())
			}
		}

		let header = self.read_header()?;
		#[cfg(test)]
		debug!("{:?}", header);


		// TODO
		// For some reason, reading the header reads 40 bytes, while writing 36 bytes(u32 + u128*2)
		// Changed u128 to u64, for some reason it reads 24 instead of 20
		self.reader.seek(SeekFrom::Start(20))?;

		// lu map<[u8], (records start, records length, records count)>
		let (lu_map, declarations) = if header.decl_len > 0 {
			self.read_declarations(header.decl_len).expect("[Reading declarations] Failed to declarations")
		} else {
			(HashMap::new(), DeclarationMap::new())
		};

		let kv_records = if header.records_len > 0 {
			self.read_kv_records(header.records_len).expect("[Reading KV records] Failed to read KV records")
		} else {
			HashMap::new()
		};

		let decl_records = if header.decl_records_len > 0 {
			let mut gr = DeclarationRecords::new();

			for (name, _) in declarations.iter() {
				// rstart, rlen, rcount
				let (rstart, rlen, _) = lu_map[name];
				self.reader.seek(SeekFrom::Start(rstart))?;
				let records = self.read_decl_records(rlen).expect("[Reading decl records] Failed to read decl records");

				gr.insert(name.to_vec(), records);
			}

			gr
		} else {
			let mut hm = HashMap::new();

			for key in declarations.keys() {
				hm.insert(key.to_vec(), Vec::new());
			}

			hm
		};


		#[cfg(test)]
		debug!("Loaded {} Declarations, {} KV records, {} Declared records in {:?}",
			declarations.len(), 
			kv_records.len(), 
			decl_records.values().map(|v| v.len()).sum::<usize>(),
			time.elapsed());
		
		*memory.decl_get_mut() = declarations;
		*memory.kv_records_get_mut() = kv_records;
		*memory.decl_records_get_mut() = decl_records;

		Ok(())
	}

	pub fn read_header(&mut self) -> std::io::Result<Header> {
		let header: Header = {
			let mut hbuf = [0u8; std::mem::size_of::<Header>()];
			self.reader.read_exact(&mut hbuf)?;
			deserialize(&hbuf)
		};

		Ok(header)
	}

	pub fn read_declarations(&mut self, len: u32) -> std::io::Result<(HashMap<Vec<u8>, (u64, u64, u64)>, DeclarationMap)> {
		let mut dbuf = vec![0u8; len as usize];
		self.reader.read_exact(&mut dbuf).expect("[Reading declarations]");
		#[cfg(test)]
		debug!("[Reading declarations] Read {}/{}", dbuf.len(), len);
		Ok(parser::decl::extract_decls(&dbuf))
	}

	pub fn read_kv_records(&mut self, len: u64) -> std::io::Result<HashMap<Vec<u8>, BvObject>> {
		let mut dbuf = vec![0u8; len as usize];
		self.reader.read_exact(&mut dbuf)?;
		#[cfg(test)]
		debug!("[Reading kv records] Read {}/{}", dbuf.len(), len);
		Ok(parser::kv::extract(&dbuf))
	}

	pub fn read_decl_records(&mut self, len: u64) -> std::io::Result<Vec<DeclarationRecord>> {
		let mut dbuf = vec![0u8; len as usize];
		self.reader.read_exact(&mut dbuf).expect("[Reading Decl records]");
		#[cfg(test)]
		debug!("[Reading decl records] Read {}/{}", dbuf.len(), len);
		Ok(parser::decl::records::extract(&dbuf))
	}
}



