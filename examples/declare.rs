use std::io;

use icbiadb::prelude::*;
use icbiadb::{if_not_exists_declare, query, query_deserialize};


fn main() -> io::Result<()> {
	let mut db = icbiadb::mem()?;

	if_not_exists_declare!{db, "articles",
		(title: String, date: String[unique])
	};

	query!{db, "articles",
		insert (title="A short title", date="today"),
		insert (title="A short title", date="yesterday")
	};

	let mut my_records = Vec::new();
	for _x in 0..10 {
		let mut record = icbiadb::DeclarationRecord::new();
		record.insert("title", "A short title");
		record.insert("date", "today");
		my_records.push(record);
	}
	
	query!{db, "articles", insert_many my_records};

	let articles = query!{db, "articles",
		select title, date;
		filter { date == "today" && title.ends_with("title") }
	};

	let articles = query_deserialize!(articles, (title: String, date: String));

	for (title, date) in articles {
		println!("{} {}", title, date);
	}

	// Or with objects, do note there are no boundary-checks at all for neither atm
	let mut my_record = icbiadb::DeclarationRecord::new();
	my_record.insert("title", "A short title");
	my_record.insert("date", "today");
	db.decl_insert_row("articles", my_record);

	let mut my_records = Vec::new();
	for _x in 0..10 {
		let mut record = icbiadb::DeclarationRecord::new();
		record.insert("title", "A short title");
		record.insert("date", "today");
		my_records.push(record);
	}
	db.decl_insert_many("articles", my_records);

	let mut query = db.query("articles");
	let _result = query
		.select(vec!["title", "date"])
		.filter(|record| {
			record["date"] == "today" || record["date"] == "yesterday"
		})
		.collect();

	Ok(())
}


