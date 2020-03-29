This is merely ideas that might be implemented if there are any use.

**KV storage of declarations**

A declaration rule is currently stored at the top of the file, with a start index of the records which are stored after KV records. With KV storage of declarations, the unique storage of declarations could be removed and when stored in-memory, have it's own Vec/Hashmap<S, Vec>.

I.e, ```db.declare("Article").add_field::<str>("Title")```

Which get stored as a regular KV record,

```
## In file ##
__decl__Article = HashMap<Field name, Config>

# Records
__decl_record__Article:\d = HashMap/Vec<(Field name, Value)>
```


```
## In memory ##
declarations = HashMap<Declaration name, Config>
decl_records = Vec
```

Pros:

* Makes the database less complex under the hood


Cons:

* While not yet implemented, selective reading of just a declaration and its records?
* Performence?


**Key-part configuration**

Since this would pretty much render Declarations useless except possibly having worse lookup performence, in-memory storage of the fields the field-config refers to would have to be stored seperatively to avoid unnecessarily iterations


KV records options(hashmap) of another keys limitations:

```db.store("key_conf{article:*:text}", {type: "String", "unique": true});```

Which can then be built upon for dynamic data-collections

**Example**

article:\d:title => {"type": "String", "len": "255"},

article:\d:text => {"type": "String"},

article:\d:date => {"type": "Datetime"},

article:\d:url => {"type": "String", "len": "255", "unique": true},


