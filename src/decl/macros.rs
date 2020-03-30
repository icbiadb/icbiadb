#[macro_export]
macro_rules! query_deserialize (
	($v:expr, ($($field:ident:$type:ty),+)) => {
		$v.iter().map(|r| {
			let ($($field,)+) = r;
			($(icbiadb::deserialize::<$type>($field),)+)
		}).collect::<Vec<_>>()
	};
);

#[macro_export]
macro_rules! query {
	($db:expr, $name:literal, select $($field:ident),+;) => {{
		let mut query = $db.query($name);
		
		let v = query.records().iter()
			.map(|record| {
				let ($($field,)+) = ($(
					record.get(stringify!($field).as_bytes()).unwrap(),
				)+);

				($($field,)+)
			})
			.collect::<Vec<_>>();

		icbiadb::decl::types::QueryResult::new($db.field_map($name), v)
	}};

	($db:expr, $name:literal, select $($field:ident),+;filter $f:block) => {{
		let mut query = $db.query($name);
		let lam_filter = |$($field:&icbiadb::types::ByteVec),+| $f;

		let v = query.records().iter()
			.filter_map(|record| {
				let ($($field,)+) = ($(
					record.get(stringify!($field).as_bytes()).unwrap(),
				)+);

				if lam_filter($($field.into(),)+) { return Some(($($field,)+)) }
				None
			})
			.collect::<Vec<_>>();
		
		icbiadb::decl::types::QueryResult::new($db.field_map($name), v)
	}};

	($db:expr, $name:literal, $(insert ($($key:ident=$val:expr),+)),+) => {
		$(
			let row = vec![$((stringify!($key).as_bytes().to_vec(), icbiadb::serialize($val).into())),+].iter().cloned()
				.collect::<std::collections::HashMap<_, _>>();
			$db.memory_mut().decl_insert_row($name, icbiadb::decl::types::DeclarationRecord::new(row));
		)+
	};
}










