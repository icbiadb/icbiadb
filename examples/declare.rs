use std::io;

use icbiadb::{query, query_deserialize};


fn main() -> io::Result<()> {
	let mut db = icbiadb::mem()?;

	if !db.has_decl("articles") {
		let mut articles = db.declare("articles");

		articles
			.add_field::<str>("title")
				.option("unique", true)
			.add_field::<str>("date");

		db.insert_decl(&articles);
	}

	query!{db, "articles",
		insert (title="A short title", date="today"),
		insert (title="A short title", date="yesterday")
	};

	let articles = query!{db, "articles",
		select title, date;
		filter { date == "today" }
	};

	let articles = query_deserialize!(articles, (title: String, date: String));
	println!("{:?}", articles);

	Ok(())
}


