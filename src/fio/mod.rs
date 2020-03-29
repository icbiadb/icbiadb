
pub mod writer;
pub mod reader;


use std::{
	io::{BufReader, BufWriter},
	sync::RwLock,
}; 

use crate::mem::Memory;
use writer::Writer;
use reader::Reader;

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

	pub fn read_to(&self, mut memory: &mut Memory) -> std::io::Result<()> {
		let mut reader = self.reader.write().unwrap();
		reader.read_to(&mut memory)
	}

	pub fn dump_mem(&mut self, mem: &Memory) -> std::io::Result<()> {
		let mut writer = self.writer.write().unwrap();
		writer.dump_memory(mem)
	}
}
