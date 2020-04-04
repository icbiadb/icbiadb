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

	if rv.starts_with("mut ".as_bytes()) {
		rv = &rv["mut ".len()..];
	} else if rv.starts_with("const ".as_bytes()) {
		rv = &rv["const ".len()..];
	}

	rv
}

pub fn contains_sequence(v: &[u8], seq: &[u8]) -> bool {
	if seq.len() > v.len() {
		return false
	}

	let mut seq_i = 0;
	for byte in v {

		if *byte == seq[seq_i] {
			seq_i += 1;
		} else {
			// Check if current non-equal byte instead equals the first seq byte, then we from there
			// To avoid a bug where it fails to find some sequences, eg,
			// v 		seq
			// 100 		100
			// 100 		111 Fail, start over
			// 111		100 Fail
			if *byte == seq[0] {
				seq_i = 1;
			} else {
				seq_i = 0;
			}
		}

		if seq_i == seq.len() {
			return true
		}
	}

	false
}




