//! A lightweight database implementation for KV, table and document databases.
//!
//! # Key-Value example
//!
//! ```
//! let db = icbiadb::kv::create::<icbiadb::IndexedKvStorage>("my_kvs.idb");
//! db.set("hello:world", 100);
//! db.commit();
//! ```
//!
//! # Table example
//!
//! ```
//! let db = icbiadb::table::create("my_tables.idb");
//! db.set("hello:world", 100);
//! db.commit();
//! ```
//!
//! # Document example
//!
//! ```
//! let db = icbiadb::doc::create("my_docs.idb");
//! db.set("hello:world", 100);
//! db.commit();
//! ```

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
