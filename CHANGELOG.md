## Changelog


### 0.3.7, 2021-07-09

**Changes**

* Truncate database files before commit


### 0.3.6, 2021-07-08

**Key-Value db**

* Remove println left behind in release


### 0.3.5, 2021-06-30

**Changes**

**Key-Value db**

* Remove file stamp when passing buffer to parser::extract_records


### 0.3.4, 2021-06-30

**Changes**

**Key-Value db**

* Added kv::read_from
* Added KvDb::commit_to
* Update stamp length when reading kvdb files


### 0.3.3, 2021-06-22

**Changes**

**Key-Value db**

* Disabled IndexedKv storage
* Added export/import interface to storages and KvDb


---


### 0.3.2, 2020-09-21


**Changes**

* Remove string tag from bytevec before converting to BvObj::as_str
* KvDb::del & KvInterface::remove now returns Option<BvObject>
* KvDb::set_raw no longer serialize BvObject
* Add KvInterface::with_capacity
* Add BvObj::as_str_slice
* Add BvObject::as_str_slice


---


### 0.3.0, 2020-07-19


**Breaking changes**
* *icbiadb::Db* has been replaced with *KvDb*, *TableDb*, *DocDb*

**Changes**

**Key-Value db**

* Impl BTreeMap storage
* Add *KvDb.get_str* -> *BvStr*
* Add *KvDb.key_regex*, *key_regexset*, *value_regex*(str only), *value_regexset*(str only), optional "regex_search" feature


**Table db**
* Change TableRow.insert to TableRow.set_col


---


### 0.2.3, 2020-04-26


* *Db.decl_insert_many* now extends instead of draining and using Db.decl_insert_row
* Add *Db.incr*, *decr*, *incr_by*, *decr_by*
* Add *BvTuple.get*(index) -> *BvObj*
* Add *BvObj*, *BvStr*, *BvInt*
* Add *ByteVec* i8, u8 conversion
* Move *Db.update* to *Db.set*
* Rename *store*, *fetch* -> *set*, *get*
* Remove *Db::write_only*
* *Db.remove* now returns the removed element
* *Db.set* now checks byte vec length for fast value replace, else remove and insert new value
* Impl *BvObject.PartialEq*
* Fix crash when committing an empty db
* Fix error result when committing with a file name
* Indexed kv storage, fix iterate bug missing last index and index reset


---


### 0.2.2 2020-04-06


* Add BvTuple and atomic change of element value if equal type and length 
* Impl generic storage for KV records 
* Impl a first-byte index vector storage for KV records
* Impl BvStr, borrows &[u8]
* BvObject now derefs to BvObject.raw
* Add slice helper functions(find, find_all, split)
* Added MemState and icbiadb::write_only initialization
* Add Db.query().new_row, just creates a DeclarationRecord::new()
* Db.update now checks length and simply overwrite the KV value
* Add Db.swap, replace KV value if serialized byte vec is of equal lengths
* Memory, all functions now takes &[u8] as parameter instead of strings, the Db layer converts strings to bytes
* slice::strip_ref_symbols now removes \*const & \*mut as well


---


### 0.2.1 2020-04-2


* BvObject cmp::PartialEq<&[u8]>, remove str len from self if is_str else compare whole slice
* Fixed crash when reading a db-file with only declarations, fio/writer wrote decl::records::IDENT even if there weren't any records, adding written length to decl_records_len in the header making the reader try to parse decl records


---


### 0.2.0 2020-04-02

* Bump version, crates.io didn't sort 0.1.3 as latest when 0.1.21 existed


### 0.1.3, 2020-04-02


**Breaking changes**

* Got rid of Record all together
* Added BvObject which now holds type name and byte vec of object, use BvObject.extract::<type>() for deserialization
* Db.fetch now returns BvObject
* Db.filter now takes |(BvKey, BvObject)| as closure parameter and returns Vec<(&BvKey, BvObject)>
* Db.starts_with, Db.contains and Db.ends_with now returns (&k, &v), i.e Vec<(&BvString, &BvObject)>
* QueryBuilder now uses BvObject instead of ByteVec


**Changes**

* Added type & unique boundary check for declarations
* Add f32(-> f64), usize(-> u64) & isize(-> i64) to normalize_type_name
* move log and env_logger to dev-dependencies
* Added ByteVec PartialCmp\<BvString\> & \<&BvString\> and more...
* Add boundary check for declaration rows


---


### 0.1.21, 2020-04-01


* Added if_not_exists_declare! macro
* Changed DeclarationRecord, easier to construct and insert without macro, see example
* Added query!{ insert_many Vec\<DeclarationRecord\> }
* Added Db.decl_insert_row(DeclarationRecord) and decl_insert_many(Vec<DeclarationRecord>)
* Removed BorrowedDeclRecord & DeclValue
* Db.query.select.filter now uses a HashMap<&str, &ByteVec>, <field name, value> for closure parameter(i.e, able to use ByteVecs int & str comparisons)
* No longer use Db.mem_mut().decl_insert_* in macros, in the future, Db will do boundary checks before inserting into memory
* Added ByteVec.extract for deserialization


Will probably take a look at src/decl/ in the weekend/next week and probable change the whole thing/do some major changes



