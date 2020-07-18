This is merely ideas that might be implemented if there are any use.


**Modularized KV/SQL/Document interface**


```rust
let mut db = icbiadb::Database::create("test.idb");



// KV database
let kv = db.kv_storage::<IKVStorage>();
//let kv = db.kv_storage::<BinTree>();
kv.set("test1", 0);
kv.filter(|(k, v)| {
	v == 0
});


// SQL database
let sql = db.sql_storage();

if_not_exists_create!{sql, "test",
	(test: String, test1: i32)
};

query!{sql, "test",
	select test, test1;
	filter {
		test1 == 0
	}
}


// JSON/BSON documents
let docs = db.document_storage();

// Customize serde_json for byte storage
let test = json!({"test": "test", "test1": 0});
docs.collection("test").insert(test);
docs.collection("test").insert_many(tests);

docs.collection("test").filter("test1" == 0);



db.commit();
```

---


**QOL moccup**

Rules for key-part identifiers.

key-base(identifier), separator, option(auto-increment, etc)
```rust
db.key_config("my_integers", ":", KvRule::AutoIncrement)
db.store("my_integers", 20) -> "my_integers:0" = 20
db.store("my_integers", 34) -> "my_integers:1" = 34
db.store("my_integers", 106) -> "my_integers:2" = 106
```

And merely just add some unique identifier for that key-rule in the keys when written to file.
```autoincr___sep:___my_integers:0,20```


Or implement a simple function for that use-case(if I don't come up with more rules)
```db.store_autoincr_or_something("my_integers", 20)...```



**Key-part configuration**


KV records options(hashmap) of another keys limitations:

```db.store("key_conf{article:*:text}", {type: "String", "unique": true});```

Which can then be built upon for dynamic data-collections

**Example**

article:\d:title => {"type": "String", "len": "255"},

article:\d:text => {"type": "String"},

article:\d:date => {"type": "Datetime"},

article:\d:url => {"type": "String", "len": "255", "unique": true},



