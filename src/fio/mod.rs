pub mod reader;
pub mod writer;

use std::{
    io::{BufReader, BufWriter, Seek, SeekFrom, Write},
    sync::RwLock,
};

use reader::Reader;
use writer::Writer;

use crate::database::{table::TableDb, DocDb, KvDb};

use crate::storage::KvInterface;
use crate::types::{BvObject, BvString};

pub struct FileIO {
    writer: RwLock<Writer<BufWriter<std::fs::File>>>,
    reader: RwLock<Reader<BufReader<std::fs::File>>>,
}

impl FileIO {
    pub fn new(f: std::fs::File) -> Self {
        FileIO {
            writer: RwLock::new(Writer::new(BufWriter::new(f.try_clone().unwrap()))),
            reader: RwLock::new(Reader::new(BufReader::new(f))),
        }
    }

    pub fn commit_kv_db<KV: KvInterface>(&mut self, kv: &KvDb<KV>) -> std::io::Result<()>
    where
        for<'a> &'a KV: IntoIterator<Item = (&'a BvString, &'a BvObject)>,
    {
        let mut writer = self.writer.write().unwrap();
        writer.write_all(b"KVIDB")?;

        for record in (&kv.records).into_iter() {
            writer.write_kv_record(record)?;
        }

        writer.flush()?;

        Ok(())
    }

    pub fn commit_table_db(&mut self, tdb: &TableDb) -> std::io::Result<()> {
        let mut writer = self.writer.write().unwrap();
        writer.curr_pos += writer.write(b"TABLEIDB")?;

        writer.curr_pos += writer.write_header()?;

        // Table definitions
        for (name, fields) in tdb.maps.iter() {
            let len = writer.write_table(name, fields)?;
            writer.curr_pos += len as usize;
            writer.table_length += len;
        }

        // Table rows
        let mut table_rows_length = 0;
        for (name, rows) in tdb.rows.iter() {
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
