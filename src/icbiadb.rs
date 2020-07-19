use crate::storage::IndexedKvStorage;

use crate::database::{DocDb, KvDb, TableDb};
use crate::fio::FileIO;

enum DbType {
    InMemory,
    File,
}

pub struct IcbiaDB {
    file_name: String,
    r#type: DbType,

    pub kvs: KvDb<IndexedKvStorage>,
    pub tables: TableDb,
    pub docs: DocDb,
}

impl IcbiaDB {
    pub fn create<S: AsRef<str>>(file_name: S) -> std::io::Result<Self> {
        let mut icbiadb = IcbiaDB {
            file_name: file_name.as_ref().to_string(),
            r#type: DbType::File,
            kvs: KvDb::default(),
            tables: TableDb::default(),
            docs: DocDb::default(),
        };

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(file_name.as_ref())?;

        let fio = FileIO::new(f);
        fio.read_file(&mut icbiadb)?;

        Ok(icbiadb)
    }

    pub fn read<S: AsRef<str>>(file_name: S) -> std::io::Result<Self> {
        let mut icbiadb = IcbiaDB {
            file_name: file_name.as_ref().to_string(),
            r#type: DbType::File,
            kvs: KvDb::default(),
            tables: TableDb::default(),
            docs: DocDb::default(),
        };

        let f = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(file_name.as_ref())?;

        let fio = FileIO::new(f);
        fio.read_file(&mut icbiadb)?;

        Ok(icbiadb)
    }

    pub fn read_to_mem<S: AsRef<str>>(file_name: S) -> std::io::Result<Self> {
        let mut icbiadb = IcbiaDB {
            file_name: file_name.as_ref().to_string(),
            r#type: DbType::File,
            kvs: KvDb::default(),
            tables: TableDb::default(),
            docs: DocDb::default(),
        };

        let f = std::fs::OpenOptions::new()
            .read(true)
            .open(file_name.as_ref())?;

        let fio = FileIO::new(f);
        fio.read_file(&mut icbiadb)?;

        Ok(icbiadb)
    }

    pub fn mem() -> std::io::Result<Self> {
        Ok(IcbiaDB {
            file_name: String::new(),
            r#type: DbType::InMemory,
            kvs: KvDb::default(),
            tables: TableDb::default(),
            docs: DocDb::default(),
        })
    }

    pub fn kv_db(&self) -> &KvDb<IndexedKvStorage> {
        &self.kvs
    }

    pub fn table_db(&self) -> &TableDb {
        &self.tables
    }

    pub fn doc_db(&self) -> &DocDb {
        &self.docs
    }

    pub fn mut_kv_db(&mut self) -> &mut KvDb<IndexedKvStorage> {
        &mut self.kvs
    }

    pub fn mut_table_db(&mut self) -> &mut TableDb {
        &mut self.tables
    }

    pub fn mut_doc_db(&mut self) -> &mut DocDb {
        &mut self.docs
    }

    pub fn kv_transaction<'a, F, T>(&'a self, cb: F) -> T
    where
        F: FnOnce(&'a KvDb<IndexedKvStorage>) -> T,
    {
        cb(&self.kvs)
    }

    pub fn table_transaction<'a, F, T>(&'a self, cb: F) -> T
    where
        F: FnOnce(&'a TableDb) -> T,
    {
        cb(&self.tables)
    }

    pub fn doc_transaction<'a, F, T>(&'a self, cb: F) -> T
    where
        F: FnOnce(&'a DocDb) -> T,
    {
        cb(&self.docs)
    }

    pub fn mut_kv_transaction<'a, F, T>(&'a mut self, cb: F) -> T
    where
        F: FnOnce(&'a mut KvDb<IndexedKvStorage>) -> T,
    {
        cb(&mut self.kvs)
    }

    pub fn mut_table_transaction<'a, F, T>(&'a mut self, cb: F) -> T
    where
        F: FnOnce(&'a mut TableDb) -> T,
    {
        cb(&mut self.tables)
    }

    pub fn mut_doc_transaction<'a, F, T>(&'a mut self, cb: F) -> T
    where
        F: FnOnce(&'a mut DocDb) -> T,
    {
        cb(&mut self.docs)
    }

    pub fn commit(&mut self) -> std::io::Result<()> {
        if self.file_name.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File name not set, are you using a memory database?",
            ));
        }
        self.r#type = DbType::File;

        let f = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(&self.file_name)?;

        let mut fio = FileIO::new(f);
        fio.write_file(&self)
    }
}
