[Latest Version]: https://img.shields.io/crates/v/icbiadb

[crates.io]: https://crates.io/crates/icbiadb
[Build Status]: https://travis-ci.com/icbiadb/icbiadb.svg?branch=master
[travis]: https://travis-ci.com/github/icbiadb/icbiadb

## IcbiaDB | I can't believe it's a database &emsp; [![Build Status]][travis] [![Latest Version]][crates.io]


**Not recommended for production**


[Changelog](https://github.com/icbiadb/icbiadb/blob/master/CHANGELOG.md)


IcbiaDB is a simple embedded 3-in-1(KV, table and JSON/BSON) database interface with JIT serialization.

The basic goal though, is merely a fast reliable database with minimal preperation, minimal dependencies and decent performence on low-end computers with the ability to seamlessly store, manipulate and present primitives and complex data structures without too much hassle. Oh, and it comes with a free beer.


**Features**


**KV**:
* Multiple data storages
* Atomic operations on tuples, integers and strings
* Filter by key, type name or value with or without regex(See "regex_search" feature)


**Tables**:


**JSON**:
Not implemented yet


**Example**


```rust
use serde::{Deserialize, Serialize};

use icbiadb::prelude::*;
use icbiadb::types::BvObject;
use icbiadb::{if_not_exists_create, query};
use icbiadb::storage::BTreeMap;

#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    text: String,
}

fn main() -> std::io::Result<()> {
    let mut db = icbiadb::kv::mem::<BTreeMap>();

    // set, get
    db.set("key:welcome", "Hello World!");
    let v = db.get("key:welcome").unwrap(); // -> BvObject

    if v == "Hello World!" || v == 100 {
        println!("{:?} of type {}", v.extract::<String>(), v.type_name());
    }

    db.set("key:welcome", 100);
    let key_welcome = db.get_value::<i32>("key:welcome");

    if db.get("visited").is_some() {
        db.incr("visitors");
    }

    let article = Article {
        title: "A title".to_string(),
        text: "Hello World!".to_string(),
    };
    db.set("articles:0", &article);

    // Atomic operations on tuple elements, requires same type and length.
    db.set("my_tuple", (100, 100, "hello world!"));
    let mut bvtuple = db.get_tuple("my_tuple").unwrap();
    bvtuple.set(1, 111); // -> (100, 111, "hello world!")
    bvtuple.set(2, "hello!!!!!!!");
    bvtuple.value::<i32>(1); // -> 111

    // Seamless string bytes comparison, integers are atm converted natively(from_le_bytes)
    db.filter(|(k, v)| v.type_name() == "IcbiaDB_tests::Article" || v.contains("this is a string"));

    db.starts_with("calculations:")
        .iter()
        .filter(|(k, v)| k.contains(":super_calc:") && *v > 100.0 && *v < 200.0)
        .collect::<Vec<_>>();

    Ok(())
}
```

---


**Serialization/Deserialization**

Serialization & deserialization is not necessarily a slow procedure, but for low-end computers, valuable CPU time could be spent on reading/parsing the next record instead of mass-deserialization of data, which is a contributing factor to delays just like some operations on higher-level data types. Let's take care of stupid things like converting bytes to regular easy-to-handle types after it has been filtered, manipulated, ordered and about to be presented instead! 

... IF it's not better to deserialize it earlier, then whatever. In other words, IcbiaDB _aims for_ JIT, mass- and just-once deserialization after thinning out the number of records.


**Unsupported types**

IcbiaDB stores everything as simple byte arrays and don't mess with it once its been loaded into memory, so it's possible to serialize your complex structure in your chosen language, store it raw, maybe manipulate it with the built-in byte utilities or your own, and deserialize it without any interference from IcbiaDB.


---


**NOTE**

Coming soon

* substr/srange(start, length)
* More REDIS-inspired stuff


---


**IcbiaDB in other languages**


Since this is a rather enjoyable project, if my time allows it, I plan to extend it into other languages.

* C ffi, not yet written
* [icbiadb-py](https://github.com/icbiadb/icbiadb-py)
* Nodejs not written yet, [neon](https://github.com/neon-bindings/neon)
* Ruby not written yet, [rutie](https://github.com/danielpclark/rutie)
* Lua not written yet, [rlua](https://github.com/kyren/rlua)

