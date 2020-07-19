//! A table database implementation
//!
//! # Example
//!
//! ```
//! // You can have rust code between fences inside the comments
//! // If you pass --test to `rustdoc`, it will even test it for you!
//! use doc::Person;
//! let person = Person::new("name");
//! ```

pub mod parser;
pub mod types;

use std::io::{BufReader, Seek, SeekFrom};

use crate::fio;
use types::*;

pub fn mem() -> TableDb {
    TableDb {
        file_name: String::new(),
        maps: TableMap::default(),
        rows: TableRows::default(),
    }
}

pub fn create(file_name: &str) -> std::io::Result<TableDb> {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;

    let mut reader = fio::reader::Reader::new(BufReader::new(f));

    if reader.is_empty() {
        return Ok(TableDb {
            file_name: file_name.to_string(),
            maps: TableMap::default(),
            rows: TableRows::default(),
        });
    }

    reader.seek(SeekFrom::Start(8))?;
    let header = reader.read_header()?;
    let (lu_map, tmaps) = reader.read_table_definitions(header.table_length)?;

    let mut trows = TableRows::new();

    for (name, _) in tmaps.iter() {
        // rstart, rlen, rcount
        let (rstart, rlen, _) = lu_map[name];

        reader.seek(SeekFrom::Start(rstart))?;
        let records = reader
            .read_table_rows(rlen)
            .expect("[Reading decl records] Failed to read decl records");

        trows.insert(name.to_vec(), records);
    }

    Ok(TableDb {
        file_name: file_name.to_string(),
        maps: tmaps,
        rows: trows,
    })
}

#[derive(Default)]
pub struct TableDb {
    pub file_name: String,
    pub maps: TableMap,
    pub rows: TableRows,
}

impl TableDb {
    pub fn commit(&self) -> std::io::Result<()> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file_name)?;

        let mut fio = fio::FileIO::new(f);
        fio.commit_table_db(self)?;

        Ok(())
    }

    pub fn exists<S: AsRef<str>>(&self, name: S) -> bool {
        self.maps.contains_key(name.as_ref().as_bytes())
    }

    pub fn new_table<S: AsRef<str>>(&self, name: S) -> Table {
        Table::new(name)
    }

    pub fn create(&mut self, table: Table) {
        self.maps
            .insert(table.raw_name().to_vec(), table.fields().to_owned());
        self.rows.insert(table.raw_name().to_vec(), Vec::new());
    }

    pub fn remove<S: AsRef<str>>(&mut self, name: S) {
        self.maps.remove(name.as_ref().as_bytes());
        self.rows.remove(name.as_ref().as_bytes());
    }

    pub fn map<S: AsRef<str>>(&self, name: S) -> &FieldMap {
        &self.maps[name.as_ref().as_bytes()]
    }

    pub fn rows<S: AsRef<str>>(&self, name: S) -> &Vec<TableRow> {
        &self.rows[name.as_ref().as_bytes()]
    }

    pub fn insert_row<S: AsRef<str>>(&mut self, name: S, row: TableRow) -> Result<(), String> {
        let table_map = &self.maps[name.as_ref().as_bytes()];
        let rows = &self.rows[name.as_ref().as_bytes()];

        let mut unique_values = std::collections::HashMap::new();
        let unique_fields: Vec<Vec<u8>> = table_map
            .iter()
            .filter_map(|(k, v)| {
                if v.contains_key("unique".as_bytes()) {
                    Some(k.to_vec())
                } else {
                    None
                }
            })
            .collect();

        for (k, v) in table_map.iter() {
            if v["type".as_bytes()] != row[k].type_name() {
                return Err(format!(
                    "Expected type \"{}\" found type \"{}\"",
                    v["type".as_bytes()].as_str(),
                    row[k].type_name()
                ));
            }

            if v.contains_key("unique".as_bytes()) {
                unique_values.insert(k.to_vec(), &row[k]);
            }
        }

        for srow in rows.iter() {
            for unique_field in unique_fields.iter() {
                if srow[unique_field.as_slice()] == unique_values[unique_field.as_slice()] {
                    return Err(format!(
                        "\"{}\" collided",
                        std::str::from_utf8(unique_field).unwrap()
                    ));
                }
            }
        }

        self.rows
            .entry(name.as_ref().as_bytes().to_vec())
            .and_modify(|v| v.push(row));
        Ok(())
    }

    pub fn insert_many<S: AsRef<str>>(
        &mut self,
        name: S,
        new_rows: Vec<TableRow>,
    ) -> Result<(), String> {
        let table_map = &self.maps[name.as_ref().as_bytes()];
        let rows = &mut self
            .rows
            .get_mut(name.as_ref().as_bytes())
            .unwrap_or_else(|| panic!("Failed to get rows of table {:?}", name.as_ref()));

        for row in new_rows.iter() {
            for (k, v) in table_map.iter() {
                if v["type".as_bytes()] != row[k].type_name() {
                    return Err(format!(
                        "Expected type \"{}\" found type \"{}\"",
                        v["type".as_bytes()].as_str(),
                        row[k].type_name()
                    ));
                }

                if v.contains_key("unique".as_bytes()) {
                    for record in rows.iter() {
                        println!("{:?}", record);
                        if record[k] == row[k] {
                            panic!(
                                "Collided {}: {}",
                                std::str::from_utf8(k).unwrap(),
                                row[k].as_str()
                            );
                        }
                    }
                }
            }
        }

        rows.extend(new_rows);

        Ok(())
    }

    pub fn query<S: AsRef<str>>(&self, name: S) -> QueryBuilder {
        let table_map = &self.maps[name.as_ref().as_bytes()];
        let rows = &self.rows[name.as_ref().as_bytes()];
        QueryBuilder::new(table_map, rows)
    }
}
