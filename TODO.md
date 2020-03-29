
### TODO


* Cached single-time deserialization for records
* ~~Mass-deserialization for collection of records~~(See macros::query_deserialize)
* Nicer error-handling/more helpful panics
* Redeclare declarations, add/remove columns, rename, change types, return records that don't fit the changes for deletion/updating(Might be useful later, if a script language is ever created for some db stuff or something)
* Implement some kind of basic ACID transactions
* File-based sessions
* ~~Query interface for declarations and god forbid, KV records?~~(See macros::query for decl queries), KV coming soon?
* Regex and range symbols for key filtering? E.g db.fetch("test:\d{3}"), db.fetch("test:10..20")
* Closure record updates
* Async feature
* Migration functionality
* ~~Implement declarations~~
* ~~No references in stored type names~~ Fine-tuning needed though(\*const & \*mut isn't handled)
* Atomic support for KV(hashmap), Slices/Vecs and tuple fields(?)
* Atomic support on declaration records(search/select(fields?), filter, order(desc & ascd), limit)
* Deref record with JIT/cached deserialization?
* ~~Consistent name and calling conventions for declarations, declaration records~~
* Speed up reading & parsing big data sets
* Clean everything up
* Optimize everything
* Steal more ideas from Redis, MySQL and other DBs

**Macros**

* Macro for defining a new declaration
* [query] insert_many (Vec<HashMap> & Vec<()>)
* [query] insert (HashMap & Vec<()>)
* [query] allow multiple filters
* [query] Implement order_by, group_by?
* [query_deserialize] change params to only take type instead of $field:ident:$t:ty(ident used for tuple deconstruct atm, alternatives?)


**KV**

* Search & filter only of types T(Create type name lookup maps), like redis HSCAN, SSCAN etc


**Declarations**

* Validate and check declarations field-rules for mem::decl_insert_row
* Impl update
* Type check/validation for field options in field_map
* Column options(Primary Key, Not null, Foreign key?, Unique)
* ~~Querable~~

* Byte slices helper functions(strip, count, compare etc...)

..* Strip
..* Count
..* Compare

* Byte & bitwise operations on byte arrays of some types, filter/search/comparison/ordering/manipulation

..* String manipulation, comparison, char/word/seq search
..* Integer comparison(and add, sub etc?)
..* Datetime manipulation & comparison(?)


---

