#![allow(bare_trait_objects, unused_macros)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use] extern crate log;
#[cfg(feature = "alloc")]
extern crate alloc;
extern crate bincode;
extern crate serde;
#[cfg(feature="async-std-comp")]
#[macro_use] extern crate async_trait;
#[cfg(feature="async-std-comp")]
extern crate async_std;
#[cfg(feature="async-std-comp")]
extern crate futures;

pub mod utils;
pub mod decl;
pub mod prelude;
pub mod parser;
pub mod db;
pub mod slice;
pub mod types;
pub mod macros;
mod mem;
mod fio;

pub use db::Db;
pub use decl::types::DeclarationRecord;
pub use utils::{serialize, deserialize, serialize_to_bytevec, deserialize_bytevec};


pub fn create<S: AsRef<str>>(file_name: S) -> std::io::Result<db::Db> {
	db::Db::create(file_name)
}

pub fn read<S: AsRef<str>>(file_name: S) -> std::io::Result<db::Db> {
	db::Db::read(file_name)
}

pub fn read_to_mem<S: AsRef<str>>(file_name: S) -> std::io::Result<db::Db> {
	db::Db::read_to_mem(file_name)
}

pub fn mem() -> std::io::Result<db::Db> {
	db::Db::mem()
}
