//! A lightweight database implementation for KV, table and document databases.
//!
//! # Notes
//!
//! Bincode serialization defaults integers to i32/u32 and f32 for floats without any explicit declarations
//!
//! # Key-Value example
//!
//! ```
//! use icbiadb::storage::BTreeMap;
//!
//! let mut db = icbiadb::kv::create::<BTreeMap>("my_kvs.idb").unwrap();
//! db.set("hello:world", 100);
//! db.commit();
//! ```
//! See [KvDb](database/kv/struct.KvDb.html) for all methods.
//!
//! # Table example
//!
//! ```
//! use icbiadb::if_not_exists_create;
//!
//! let mut db = icbiadb::table::create("my_tables.idb").unwrap();
//!
//! if_not_exists_create! {db, "articles",
//!     (title: String, date: String[unique])
//! };
//!
//! let mut record = icbiadb::TableRow::default();
//! record.set_col("title", "A short title");
//! record.set_col("date", "today");
//! db.insert_row("articles", record);
//!
//! db.commit();
//! ```
//!

#[cfg(test)]
#[macro_use]
extern crate log;
extern crate bincode;
extern crate serde;

mod byte_size;
pub mod database;
pub mod fio;
pub mod macros;
pub mod prelude;
pub mod slice;
pub mod storage;
pub mod types;
pub mod utils;

pub use database::{
    kv,
    table::{self, types::TableRow},
    {DocDb, KvDb, TableDb},
};
pub use utils::{
    deserialize, deserialize_bytevec, deserialize_object, normalize_type_name, serialize,
    serialize_object, serialize_to_bytevec,
};
