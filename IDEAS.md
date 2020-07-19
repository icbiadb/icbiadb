This is merely ideas that might be implemented if there are any use.


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



