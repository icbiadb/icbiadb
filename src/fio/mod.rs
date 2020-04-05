
pub mod writer;
pub mod reader;


use std::{
	io::{BufReader, BufWriter},
	sync::RwLock,
}; 

use crate::mem::Memory;
use writer::Writer;
use reader::Reader;
use crate::storage::KvInterface;
use crate::types::BvObject;

pub struct FileIO {
	writer: RwLock<Writer<BufWriter<std::fs::File>>>,
	reader: RwLock<Reader<BufReader<std::fs::File>>>,
}

impl FileIO {
	pub fn new(f: std::fs::File) -> Self {
		//let initl_buff_size = f.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);

		FileIO {
			writer: RwLock::new(Writer::new(BufWriter::new(f.try_clone().unwrap()))),
			reader: RwLock::new(Reader::new(BufReader::new(f))),
		}
	}

	pub fn read_to<KV: KvInterface<Key=Vec<u8>, Value=BvObject, RefKey=[u8]>>(&self, mut memory: &mut Memory<KV>) -> std::io::Result<()> {
		let mut reader = self.reader.write().unwrap();
		reader.read_to(&mut memory)
	}

	/*
	Higher ranked trait bounds (HRTB for short)
	Here you can pretty much read it as "for any possible lifetime 'a"
	*/
	pub fn dump_mem<KV>(&mut self, mem: &Memory<KV>) -> std::io::Result<()> 
			where KV: KvInterface<Key=Vec<u8>, Value=BvObject, RefKey=[u8]>,
			for<'a> &'a KV: IntoIterator<Item = &'a (Vec<u8>, BvObject)> {
		let mut writer = self.writer.write().unwrap();
		writer.dump_memory(mem)
	}
}
