use std::io;

use icbiadb::prelude::*;
use icbiadb::{if_not_exists_create, query, query_deserialize};

fn main() -> io::Result<()> {
    let mut db = icbiadb::table::mem();

    if_not_exists_create! {db, "articles",
        (title: String, date: String[unique])
    };

    query! {db, "articles",
        insert (title="A short title", date="today"),
        insert (title="A short title", date="yesterday")
    };

    let mut my_records = Vec::new();
    for _x in 0..10 {
        let mut record = icbiadb::TableRow::default();
        record.insert("title", "A short title");
        record.insert("date", "today");
        my_records.push(record);
    }

    query! {db, "articles", insert_many my_records};

    let articles = query! {db, "articles",
        select title, date;
        filter { date == "today" && title.ends_with("title") }
    };

    let articles = query_deserialize!(articles, (title: String, date: String));

    for (title, date) in articles {
        println!("{} {}", title, date);
    }

    // Or with objects
    let mut my_record = db.query("articles").new_row();
    my_record.insert("title", "A short title");
    my_record.insert("date", "today");
    db.insert_row("articles", my_record);

    let mut my_records = Vec::new();
    for _x in 0..10 {
        let mut record = icbiadb::TableRow::default();
        record.insert("title", "A short title");
        record.insert("date", "today");
        my_records.push(record);
    }
    db.insert_many("articles", my_records);

    let mut query = db.query("articles");
    let _result = query
        .select(vec!["title", "date"])
        .filter(|record| record["date"] == "today" || record["date"] == "yesterday")
        .collect();

    Ok(())
}
