
## TODO


* Add Search/Filter result where starts_with, contains, ends_with, key_regex, value_regex can be used multiple times
* Separate BvObject and ByteVec string operations, i.e, stripping string length set by bincode::serialize, since ByteVec is also used for wrapping String.as_bytes and such
* Operations requiring resize on tuples/vec?
* Data deduplication?
* Cached single-time deserialization for records
* Nicer error-handling/more helpful panics
* Implement some kind of basic ACID transactions
* File-based sessions
* Async feature
* Migration functionality
* Deref record with JIT/cached deserialization?


**Macros**

* [query] Implement order_by, group_by?
* [query_deserialize] change params to only take type instead of $field:ident:$t:ty(ident used for tuple deconstruct atm, alternatives?)


### Storages

**IndexedKvStructure**

* Key seperated multi-indexed Vec(e.g "article:title:hashid")
* Impl insert_many

### Databases

**KV**

* Search & filter only of type T(Create type name lookup maps), like redis HSCAN, SSCAN etc


**Table**

* Impl update
* Column options(Primary Key, Foreign key?, Unique)

* Byte slices helper functions
..* Strip
..* Count
..* Compare

* Byte & bitwise operations on byte arrays of some types
..* String manipulation, comparison, char/word/seq search
..* Integer comparison(and add, sub etc?)
..* Datetime manipulation & comparison(?)


---





