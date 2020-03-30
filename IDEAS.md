This is merely ideas that might be implemented if there are any use.


**QOL moccup**

Rules for key-part identifiers, might be added regardless of replacing declarations with kv records collections.

key-base(identifier), separator, option(auto-increment, ..)
```
db.key_config("my_integers", ":", KvRule::AutoIncrement)
db.store("my_integers", 20) -> "my_integers:0" = 20
db.store("my_integers", 34) -> "my_integers:1" = 34
db.store("my_integers", 106) -> "my_integers:2" = 106
```

And merely just add some unique identifier for that key-rule in the keys when written to file.
```autoincr___sep:___my_integers:0,20```


Or implement a simple function for that use-case(if I don't come up with more rules)
```db.store_autoincr_or_something("my_integers", 20)...```



**Database state for input-only**

Rather specific use-case, i.e usefull when you create a app only meant to generate databases, create a state that disables all unnecessary functionality(like lookup maps updating when using store/insert and so on)


**KV storage of declarations**

Declaration rules are currently stored at the top of the file, with a start index for the records which are stored after KV records. With KV storage of declarations, all that could be scrapped and only have in-memory separation.

I.e, ```db.declare("Article").add_field::<str>("Title")```

Which get stored as a regular KV record,

```
## In file ##
__decl__Article = HashMap<Field name, Config>

# Records
__decl_record__Article:\d = HashMap/Vec<(Field name, Value)>
```


Pros:

* Makes the database less complex under the hood


Cons:

* While not yet implemented, selective reading of just a declaration and its records?
* Performence?


**Key-part configuration**


KV records options(hashmap) of another keys limitations:

```db.store("key_conf{article:*:text}", {type: "String", "unique": true});```

Which can then be built upon for dynamic data-collections

**Example**

article:\d:title => {"type": "String", "len": "255"},

article:\d:text => {"type": "String"},

article:\d:date => {"type": "Datetime"},

article:\d:url => {"type": "String", "len": "255", "unique": true},



