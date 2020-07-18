pub mod parser;
pub mod types;

use types::*;

#[derive(Default)]
pub struct TableDb {
    pub maps: TableMap,
    pub rows: TableRows,
}

impl TableDb {
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
