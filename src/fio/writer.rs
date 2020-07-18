use std::collections::HashMap;
use std::io::SeekFrom;

use crate::byte_size::globals::*;
use crate::database::table::types::*;
use crate::types::bv::{BvObject, BvString};
use crate::utils::serialize;

pub struct Writer<T: std::io::Write + std::io::Seek> {
    pub writer: T,
    pub curr_pos: usize,
    pub table_length: u32,
    pub kv_records_length: u64,
    pub table_rows_length: u64,
    pub decl_lu_map: HashMap<Vec<u8>, u64>,
}

impl<T: std::io::Write + std::io::Seek> Writer<T> {
    pub fn new(writer: T) -> Self {
        Writer {
            writer,
            curr_pos: 0,
            table_length: 0,
            kv_records_length: 0,
            table_rows_length: 0,
            decl_lu_map: HashMap::new(),
        }
    }

    pub fn write_header(&mut self) -> std::io::Result<usize> {
        let mut length = 0;
        self.writer.seek(SeekFrom::Start(0))?;
        length += self.writer.write(&serialize(&self.table_length))?;
        length += self.writer.write(&serialize(&self.kv_records_length))?;
        length += self.writer.write(&serialize(&self.table_rows_length))?;

        Ok(length)
    }

    pub fn write_table(&mut self, name: &[u8], fields: &FieldMap) -> std::io::Result<u32> {
        // Identifier, name length, fields length, name, fields
        let ser_fields = serialize(fields);
        let mut length = 0;
        let mut curr_pos = self.curr_pos;

        length += self.writer.write(&table::IDENT)? as u32;

        length += self.writer.write(&[name.len() as u8])? as u32;
        length += self.writer.write(&serialize(&(ser_fields.len() as u16)))? as u32;

        length += self.writer.write(&name)? as u32;
        length += self.writer.write(&ser_fields)? as u32;

        let mut decl_lu_name = Vec::with_capacity(name.len() + "header_rdata_start".len());
        decl_lu_name.extend(name);
        decl_lu_name.extend(b"header_rdata_start");

        curr_pos += length as usize;
        self.decl_lu_map.insert(decl_lu_name, curr_pos as u64);
        length += self.writer.write(&serialize(&(0 as u64)))? as u32; // Records start
        length += self.writer.write(&serialize(&(0 as u64)))? as u32; // Total records length
        length += self.writer.write(&serialize(&(0 as u64)))? as u32; // Records count

        Ok(length)
    }

    pub fn write_table_rows_data(
        &mut self,
        name: &[u8],
        total_len: u64,
        records_count: u64,
    ) -> std::io::Result<()> {
        let mut decl_header_rdata_start =
            Vec::with_capacity(name.len() + "header_rdata_start".len());
        decl_header_rdata_start.extend(name);
        decl_header_rdata_start.extend(b"header_rdata_start");

        let mut decl_records_start = Vec::with_capacity(name.len() + "decl_records_start".len());
        decl_records_start.extend(name);
        decl_records_start.extend(b"decl_records_start");

        self.writer
            .seek(SeekFrom::Start(self.decl_lu_map[&decl_header_rdata_start]))?;
        self.writer
            .write_all(&serialize(&self.decl_lu_map[&decl_records_start]))?;
        self.writer.write_all(&serialize(&total_len))?;
        self.writer.write_all(&serialize(&records_count))?;

        Ok(())
    }

    pub fn write_kv_record(&mut self, record: &(BvString, BvObject)) -> std::io::Result<u64> {
        // Identifier, name length, fields length, name, fields
        let (k, v) = record;
        assert!(k.len() as u8 > 0 && v.type_name().len() as u8 > 0);
        let mut length = 0;

        length += self.writer.write(&kv::IDENT)? as u64;

        // K, TN, V lengths
        length += self.writer.write(&[k.len() as u8])? as u64;
        length += self.writer.write(&[v.type_name().len() as u8])? as u64;
        length += self.writer.write(&serialize(&(v.raw().len() as u32)))? as u64;

        // Values
        length += self.writer.write(k.as_slice())? as u64;
        length += self.writer.write(v.type_name().as_slice())? as u64;
        length += self.writer.write(v.as_slice())? as u64;
        Ok(length)
    }

    pub fn write_decl_header(&mut self) -> std::io::Result<u64> {
        let mut length = 0;
        length += self.writer.write(&table::rows::IDENT)? as u64;
        Ok(length)
    }

    pub fn write_decl_record(&mut self, hm: &TableRow) -> std::io::Result<u64> {
        let ser_hm = serialize(hm);
        let mut length = 0;

        length += self.writer.write(&table::rows::RECORD_IDENT)? as u64;
        length += self.writer.write(&serialize(&(ser_hm.len() as u32)))? as u64;
        length += self.writer.write(&ser_hm)? as u64;

        Ok(length)
    }
}

impl<T: std::io::Write + std::io::Seek> std::ops::Deref for Writer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.writer
    }
}

impl<T: std::io::Write + std::io::Seek> std::ops::DerefMut for Writer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.writer
    }
}
