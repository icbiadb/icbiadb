## Changelog



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



