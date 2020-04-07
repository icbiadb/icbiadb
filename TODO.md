
### TODO


* Data deduplication
* ~~Implement Bv[Type] wrapper for atomic operations on values(E.g, BvVec[3] == 100 retrieves and bitwise/deserializecompares serialized vectors fourth element, BvString.replace("hello", "hi") changes byte values and resizes)~~ Same type/same length updates have been implemented
* Cached single-time deserialization for records
* Nicer error-handling/more helpful panics
* Redeclare declarations, add/remove columns, rename, change types, return records that don't fit the changes for deletion/updating(Might be useful later, if a script language is ever created for some db stuff or something)
* Implement some kind of basic ACID transactions
* File-based sessions
* Regex and range symbols for key filtering? E.g db.fetch("test:\d{3}"), db.fetch("test:10..20")
* Async feature
* Migration functionality
* Atomic support on declaration records(search/select(fields?), filter, order(desc & ascd), limit)
* Deref record with JIT/cached deserialization?
* Speed up reading & parsing big data sets
* Clean everything up
* Optimize everything
* Steal more ideas from Redis, MySQL and other DBs

**IndexedKvStructure**

* Key separator seperated multi-indexing for IndexedKvStructure(up to 6 keys or something) 


**Macros**

* [query] Implement order_by, group_by?
* [query_deserialize] change params to only take type instead of $field:ident:$t:ty(ident used for tuple deconstruct atm, alternatives?)


**KV**

* Search & filter only of type T(Create type name lookup maps), like redis HSCAN, SSCAN etc


**Declarations**

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

