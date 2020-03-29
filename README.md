
## IcbiaDB | I can't believe it's a database


**Not recommended for production**

IcbiaDB is a simple headless Key-Value & data structures database with support for storing most types seamlessly. Compared to some other KV databases, IcbiaDB supports partial key searches and atomic operations without serialization, allowing for rather fast ...stuff, on big data sets.

The basic goal though, is merely a quick and dirty relatively reliable database with minimal preperation, minimal dependencies and decent performence on low-end computers and also the ability to seamlessly store, manipulate and present primitives and dynamic structures without too much hassle. Oh, and it comes with a free beer.

I.e, anything but a database library I've ever heard of, especially in this fine language.


**Key-Value records**

As mentioned above, store pretty much anything you like, from any serde::Serializeable type in rust to primitive types in supported languages to raw byte arrays, search for keys with partial key searches and filter records by type or value.


**Declarations**


The libraries for databases I've used before usually requires pre-defined struct/classes, maybe some third-party binary for setup, derives/trait macros, callbacks/registration.. or a simple query string to execute... in rust as well as other language, i.e, preperation and then even more preperation!

Screw that! In IcbiaDB, you define the nature of the structure on the spot with a few lines of code, while you're still figuring out what your data structure might consist of. You shouldn't have to think all that carefully about your database design has always been my mantra. Just type-and-go, it's almost more streamlined than World of Warcraft!


```
if !db.has_decl("articles") {
	let mut articles = db.declare("articles");

	articles
		.add_field::<str>("title")
			.option("unique", true)
		.add_field::<str>("date");

	db.insert_decl(&articles);
}
```

The declarative data structure functionality is supposed to allow for a dynamic complex data structure which can easily be supported in other languages. And no, I don't avoid using the word "table" for this highly innovative creation.

The only preparation required by IcbiaDB are for structs, they need to derive serde::Serialize since serialization and deserialization depends on bincode which depends on serde... sorry about that. Might change in the future.


**Serialization/Deserialization**

Serialization & deserialization is not necessarily a slow procedure, but for low-end computers, valuable CPU time could be spent on reading/parsing the next record instead of mass-deserialization of data, which is a contributing factor to delays as well as some operations on higher level data types. Let's take care of stupid things like converting bytes to regular easy-to-handle types after it has been filtered, manipulated, ordered and about to be presented instead! 

... IF it's not better to deserialize it earlier, then whatever. In other words, IcbiaDB _aims for_ JIT, mass- and just-once deserialization after thinning out the number of records.


**Unsupported types**

IcbiaDB stores everything as simple byte arrays and don't mess with it once its been loaded into memory, so it's possible to serialize your complex structure in your chosen language, store it raw, maybe manipulate it with the built-in byte utilities or your own, and deserialize it without any interference from IcbiaDB.


---


Example


```
use serde::{Serialize, Deserialize};

use icbiadb::prelude::*;
use icbiadb::{serialize, deserialize};



#[derive(Serialize, Deserialize)]
struct Article {
	title: String,
	text: String,
}


fn main() -> std::io::Result<()> {
	env_logger::init();

	let mut db = icbiadb::mem()?;
	
	// Store & fetch, requires icbiadb::prelude::{RecordRead}
	db.store("key:welcome", "Hello World!");
	
	let r = db.fetch("key:welcome");
	println!("Key {:?} stores {:?} of type {}", r.key(), r.value::<String>(), r.type_name());

	db.update("key:welcome", 100);
	println!("{}", db.fetch_value::<i32>("key:welcome"));

	let article = Article { title: "A title".to_string(), text: "Hello World!".to_string() };
	db.store("articles:0", &article);


	// Search & filter, requires icbiadb::prelude::{BytesSearch, BytesFilter}
	let keys = db.starts_with("key:");

	let articles = db.filter(|r| {
		r.raw_type_name() == "IcbiaDB_tests::Article".as_bytes()
		//With serialization, r.type_name() == "IcbiaDB_tests::Article"
	});

	println!("Found {} keys starting with \"key:\"", keys.len());
	println!("Found {} keys of type \"Article\"", articles.len());

	Ok(())
}
```

---


**Performence**


No real bench marks yet, but as an example, searching and filtering 4 million KV records and half a million decl records on an Asus E402S, Intel Dual-Core N3060, 2gb ram, single thread

```
[2020-03-26T16:05:48Z DEBUG icbiadb::fio::reader] Loaded 0 Declarations, 4000000 KV records, 0 Declared records in 2.797347558s
[2020-03-26T16:05:51Z INFO  IcbiaDB_tests] starts_with "test:", found 999999 db: 140.30478ms
[2020-03-26T16:05:51Z INFO  IcbiaDB_tests] contains "st:1", found 111110 db: 244.240446ms
[2020-03-26T16:05:52Z INFO  IcbiaDB_tests] Filter: (r > 7000 && r < 9000) || (r > 20000 && r < 45000): 80994 items in 229.157685ms
[2020-03-26T16:05:52Z INFO  IcbiaDB_tests] Filter: r.raw_type_name() == "IcbiaDB_tests::Article".as_bytes(): 1000000 items in 181.358364ms
```

```
[2020-03-28T19:02:23Z DEBUG icbiadb::fio::reader] Loaded 1 Declarations, 0 KV records, 500000 Declared records in 4.08070582s
[2020-03-28T19:02:24Z INFO  IcbiaDB_tests] Filtered { title == "A shorter title" }: 250000 records in 193.428521ms
[2020-03-28T19:02:24Z INFO  IcbiaDB_tests] Deserialized 114 MB in 330.525201ms
```


**NOTE**


---


**IcbiaDB in other languages**


Since this is a rather enjoyable project, if my time allows it, I plan to extend it into other languages.

* C ffi, not yet written
* [icbiadb-py](https://github.com/Grundligt/icbiadb-py)
* Nodejs not written yet, [neon](https://github.com/neon-bindings/neon)
* Ruby not written yet, [rutie](https://github.com/danielpclark/rutie)
* Lua not written yet, [rlua](https://github.com/kyren/rlua)

