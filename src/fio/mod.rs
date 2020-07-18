pub mod reader;
pub mod writer;

use std::{
    collections::HashMap,
    io::{BufReader, BufWriter, Seek, SeekFrom, Write},
    sync::RwLock,
};

use reader::Reader;
use writer::Writer;

use crate::{
    database::{
        table::{types::TableRows, TableDb},
        DocDb, KvDb,
    },
    icbiadb::IcbiaDB,
};

use crate::storage::KvInterface;
use crate::types::{BvObject, BvString};

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

    pub fn read_file(&self, mut icbiadb: &mut IcbiaDB) -> std::io::Result<()> {
        let mut reader = self.reader.write().unwrap();

        if reader.is_empty() {
            return Ok(());
        }

        let header = reader.read_header()?;

        let lu_map = if header.table_len > 0 {
            let (lu_map, tmaps) = reader.read_declarations(header.table_len)?;
            icbiadb.tables.maps = tmaps;
            lu_map
        } else {
            HashMap::new()
        };

        if header.records_len > 0 {
            icbiadb.kvs.records = reader.read_kv_records(header.records_len)?;
        }

        if header.table_rows_len > 0 {
            let mut trows = TableRows::new();

            for (name, _) in icbiadb.tables.maps.iter() {
                // rstart, rlen, rcount
                let (rstart, rlen, _) = lu_map[name];
                reader.seek(SeekFrom::Start(rstart))?;
                let records = reader
                    .read_table_records(rlen)
                    .expect("[Reading decl records] Failed to read decl records");

                trows.insert(name.to_vec(), records);
            }

            icbiadb.tables.rows = trows;
        }

        Ok(())
    }

    /*
    pub fn read_kvs<KV: KvInterface<Key = BvString, Value = BvObject, RefKey = [u8]>>(
        &self,
        mut kv_db: &mut KvDb<KV>,
    ) {
        let mut reader = self.reader.write().unwrap();
    }

    pub fn read_tables(&self, mut table_db: &mut TableDb) {
        let mut reader = self.reader.write().unwrap();
    }

    pub fn read_docs(&self, mut doc_db: &mut DocDb) {
        let mut reader = self.reader.write().unwrap();
    }
    */

    pub fn write_file(&mut self, icbiadb: &IcbiaDB) -> std::io::Result<()> {
        let mut writer = self.writer.write().unwrap();

        writer.curr_pos += writer.write_header()?;

        // Table definitions
        for (name, fields) in icbiadb.table_db().maps.iter() {
            let len = writer.write_table(name, fields)?;
            writer.curr_pos += len as usize;
            writer.table_length += len;
        }

        for record in (&icbiadb.kv_db().records).into_iter() {
            writer.kv_records_length += writer.write_kv_record(record)?;
        }
        writer.curr_pos += writer.kv_records_length as usize;

        let mut table_rows_length = 0;
        for (name, rows) in icbiadb.table_db().rows.iter() {
            if !rows.is_empty() {
                let mut decl_lu_name = Vec::with_capacity(name.len() + "decl_records_start".len());
                decl_lu_name.extend(name);
                decl_lu_name.extend(b"decl_records_start");
                let pos = writer.curr_pos as u64;
                writer.decl_lu_map.insert(decl_lu_name, pos);

                table_rows_length += writer.write_decl_header()?;
                for row in rows.iter() {
                    table_rows_length += writer.write_decl_record(row)?;
                }
                writer.curr_pos += table_rows_length as usize;
                writer.table_rows_length += table_rows_length;

                writer.write_table_rows_data(name, table_rows_length, rows.len() as u64)?;
                writer.writer.seek(SeekFrom::End(0))?;
                table_rows_length = 0;
            }
        }

        writer.write_header()?;

        writer.flush()?;

        Ok(())
    }
}
