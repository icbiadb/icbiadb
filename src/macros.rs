#[macro_export]
macro_rules! query_deserialize (
	($v:expr, ($($field:ident:$type:ty),+)) => {		
		$v.iter().map(|r| {
			let ($($field,)+) = r;
			($(icbiadb::deserialize_bytevec::<$type>($field),)+)
		}).collect::<Vec<_>>()
	};
);

#[macro_export]
macro_rules! if_not_exists_declare {
	($db:expr, $name:literal, ($($key:ident:$type:ty $([$($opt:ident) +])?),+)) => {
		if !$db.has_decl($name) {
			let mut decl = $db.declare($name);
			decl
			$(
				.add_field::<$type>(stringify!($key))
				$(
					$(
						.option(stringify!($opt), true)
					)+
				)?
			)+;

			$db.insert_decl(&decl);
		}
	};
}

#[macro_export]
macro_rules! query {
	($db:expr, $name:literal, select $($field:ident),+;) => {{
		let mut query = $db.query($name);

		query.records().iter()
			.map(|record| {
				let ($($field,)+) = ($(
					record.get(stringify!($field).as_bytes()).unwrap(),
				)+);

				($($field,)+)
			})
			.collect::<Vec<_>>()
	}};

	($db:expr, $name:literal, select $($field:ident),+;filter $f:block) => {{
		let mut query = $db.query($name);
		let lam_filter = |$($field:&icbiadb::types::bv::ByteVec),+| $f;

		query.records().iter()
			.filter_map(|record| {
				let ($($field,)+) = ($(
					record.get(stringify!($field).as_bytes()).unwrap(),
				)+);

				if lam_filter($($field.into(),)+) { return Some(($($field,)+)) }
				None
			})
			.collect::<Vec<_>>()
	}};

	($db:expr, $name:literal, $(insert ($($key:ident=$val:expr),+)),+) => {
		$(
			let row = vec![$((stringify!($key).as_bytes().to_vec(), icbiadb::serialize_to_bytevec(&$val))),+].iter().cloned()
				.collect::<std::collections::HashMap<_, _>>();
			$db.decl_insert_row($name, icbiadb::decl::types::DeclarationRecord::from_hashmap(row));
		)+
	};

	($db:expr, $name:literal, insert_many $v:expr) => {
		$db.decl_insert_many($name, $v);
	};
}











