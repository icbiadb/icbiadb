use std::collections::HashMap;

use serde::Deserialize;

use crate::database::table::types::{TableMap, TableRow};
use crate::storage::KvInterface;
use crate::types::{BvObject, BvString};
use crate::utils::*;

use crate::database::{
    kv::parser::extract_records,
    table::parser::{extract_tables, rows::extract_rows},
};

#[derive(Debug, Deserialize)]
#[repr(C)]
pub struct Header {
    pub table_length: u32,
    pub table_rows_length: u64,
}

pub struct Reader<T: std::io::BufRead + std::io::Seek> {
    pub reader: T,
}

impl<T: std::io::BufRead + std::io::Seek> Reader<T> {
    pub fn new(reader: T) -> Self {
        Reader { reader }
    }

    pub fn is_empty(&mut self) -> bool {
        if let Ok(buf) = self.reader.fill_buf() {
            // Empty file
            if buf.len() == 0 {
                return true;
            }
        }

        false
    }

    pub fn read_header(&mut self) -> std::io::Result<Header> {
        let header: Header = {
            let mut hbuf = [0u8; std::mem::size_of::<Header>() - 4]; // For some reason, sizeof Header is larger than u32+u64*2(20 vs 24)
            self.reader.read_exact(&mut hbuf)?;
            deserialize(&hbuf)
        };

        Ok(header)
    }

    pub fn read_table_definitions(
        &mut self,
        len: u32,
    ) -> std::io::Result<(HashMap<Vec<u8>, (u64, u64, u64)>, TableMap)> {
        let mut dbuf = vec![0u8; len as usize];
        self.reader
            .read_exact(&mut dbuf)
            .expect("[Reading declarations]");
        #[cfg(test)]
        debug!("[Reading declarations] Read {}/{}", dbuf.len(), len);
        Ok(extract_tables(&dbuf))
    }

    pub fn read_kv_records<KV: KvInterface<Key = BvString, Value = BvObject, RefKey = [u8]>>(
        &mut self,
    ) -> std::io::Result<KV> {
        let mut dbuf = Vec::new();
        self.reader.read_to_end(&mut dbuf)?;

        if dbuf.len() <= 5 {
            Ok(KV::default())
        } else {
            #[cfg(test)]
            debug!("[Reading kv records] Read {}", dbuf.len());
            Ok(extract_records(&dbuf))
        }
    }

    pub fn read_table_rows(&mut self, len: u64) -> std::io::Result<Vec<TableRow>> {
        let mut dbuf = vec![0u8; len as usize];
        self.reader
            .read_exact(&mut dbuf)
            .expect("[Reading Decl records]");
        #[cfg(test)]
        debug!("[Reading decl records] Read {}/{}", dbuf.len(), len);
        Ok(extract_rows(&dbuf))
    }
}

impl<T: std::io::BufRead + std::io::Seek> std::ops::Deref for Reader<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.reader
    }
}

impl<T: std::io::BufRead + std::io::Seek> std::ops::DerefMut for Reader<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.reader
    }
}
