use std::io;

use icbiadb::prelude::*;
use icbiadb::{if_not_exists_declare, query, query_deserialize};


fn main() -> io::Result<()> {
	let mut db = icbiadb::mem()?;

	if_not_exists_declare!{db, "articles",
		(title: String[unique], date: String)
	};

	query!{db, "articles",
		insert (title="A short title", date="today"),
		insert (title="A short title", date="yesterday")
	};

	let articles = query!{db, "articles",
		select title, date;
		filter { date == "today" && title.ends_with("title") }
	};

	let articles = query_deserialize!(articles, (title: String, date: String));
	println!("{:?}", articles);

	Ok(())
}


