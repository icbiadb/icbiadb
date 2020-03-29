/// Slice helper functions


pub fn is_zero(s: &[u8]) -> bool {
	for b in s {
		if *b != 0 { return false }
	}

	true
}

pub fn strip_ref_symbols(v: &[u8]) -> &[u8] {
	// 38 = &, 42 = *
	let mut rv = v;
	while rv[0] == 38 || rv[0] == 42 {
		rv = &rv[1..];
	}

	rv
}






