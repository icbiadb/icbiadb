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
/*
use crate::{
    icbiadb::IcbiaDB,
    database::{DocDb, KvDb, TableDb},
};
*/

#[derive(Debug, Deserialize)]
#[repr(C)]
pub struct Header {
    pub table_len: u32,
    pub records_len: u64,
    pub table_rows_len: u64,
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

    /*
    pub fn read_to<KV: KvInterface<Key=Vec<u8>, Value=BvObject, RefKey=[u8]>>(&mut self, memory: &mut KvDb<KV>) -> std::io::Result<()> {
        // TODO, move data initialization for memory upward in the function call stack
        if let Ok(buf) = self.reader.fill_buf() {
            // Empty file
            if buf.len() == 0 {
                return Ok(())
            }
        }

        let header = self.read_header()?;

        // TODO
        // For some reason, reading the header reads 40 bytes, while writing 36 bytes(u32 + u128*2)
        // Changed u128 to u64, for some reason it reads 24 instead of 20
        self.reader.seek(SeekFrom::Start(20))?;

        // lu map<[u8], (records start, records length, records count)>
        let (lu_map, declarations) = if header.table_len > 0 {
            self.read_declarations(header.table_len).expect("[Reading declarations] Failed to declarations")
        } else {
            (HashMap::new(), DeclarationMap::new())
        };

        let kv_records = if header.records_len > 0 {
            self.read_kv_records(header.records_len).expect("[Reading KV records] Failed to read KV records")
        } else {
            KV::default()
        };

        let table_records = if header.table_rows_len > 0 {
            let mut gr = DeclarationRecords::new();

            for (name, _) in declarations.iter() {
                // rstart, rlen, rcount
                let (rstart, rlen, _) = lu_map[name];
                self.reader.seek(SeekFrom::Start(rstart))?;
                let records = self.read_table_records(rlen).expect("[Reading decl records] Failed to read decl records");

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

        *memory.table_get_mut() = declarations;
        *memory.kv_records_get_mut() = kv_records;
        *memory.table_records_get_mut() = table_records;

        Ok(())
    }
    */

    pub fn read_header(&mut self) -> std::io::Result<Header> {
        let header: Header = {
            let mut hbuf = [0u8; std::mem::size_of::<Header>() - 4]; // For some reason, sizeof Header is larger than u32+u64*2(20 vs 24)
            self.reader.read_exact(&mut hbuf)?;
            deserialize(&hbuf)
        };

        Ok(header)
    }

    pub fn read_declarations(
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
        len: u64,
    ) -> std::io::Result<KV> {
        let mut dbuf = vec![0u8; len as usize];
        self.reader.read_exact(&mut dbuf)?;
        #[cfg(test)]
        debug!("[Reading kv records] Read {}/{}", dbuf.len(), len);
        Ok(extract_records(&dbuf))
    }

    pub fn read_table_records(&mut self, len: u64) -> std::io::Result<Vec<TableRow>> {
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
