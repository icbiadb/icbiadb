## Changelog

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



