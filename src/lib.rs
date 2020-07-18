#![allow(bare_trait_objects, unused_macros)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
#[macro_use]
extern crate log;
#[cfg(feature = "alloc")]
extern crate alloc;
extern crate bincode;
extern crate serde;
#[cfg(feature = "async-std-comp")]
#[macro_use]
extern crate async_trait;
#[cfg(feature = "async-std-comp")]
extern crate async_std;
#[cfg(feature = "async-std-comp")]
extern crate futures;

mod byte_size;
pub mod database;
pub mod fio;
pub mod icbiadb;
pub mod macros;
pub mod prelude;
pub mod slice;
pub mod storage;
pub mod types;
pub mod utils;

pub use database::table::types::TableRow;
pub use icbiadb::IcbiaDB;
pub use utils::{
    deserialize, deserialize_bytevec, deserialize_object, normalize_type_name, serialize,
    serialize_object, serialize_to_bytevec,
};

pub fn create<S: AsRef<str>>(file_name: S) -> std::io::Result<icbiadb::IcbiaDB> {
    icbiadb::IcbiaDB::create(file_name)
}

pub fn read<S: AsRef<str>>(file_name: S) -> std::io::Result<icbiadb::IcbiaDB> {
    icbiadb::IcbiaDB::read(file_name)
}

pub fn read_to_mem<S: AsRef<str>>(file_name: S) -> std::io::Result<icbiadb::IcbiaDB> {
    icbiadb::IcbiaDB::read_to_mem(file_name)
}

pub fn mem() -> std::io::Result<icbiadb::IcbiaDB> {
    icbiadb::IcbiaDB::mem()
}
