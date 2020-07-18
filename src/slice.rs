/// Slice helper functions

pub fn is_zero(s: &[u8]) -> bool {
    for b in s {
        if *b != 0 {
            return false;
        }
    }

    true
}

pub fn strip_ref_symbols(v: &[u8]) -> &[u8] {
    // 38 = &, 42 = *
    let mut rv = v;
    while rv[0] == 38 || rv[0] == 42 {
        rv = &rv[1..];
    }

    if rv.starts_with(b"mut ") {
        rv = &rv[4..];
    } else if rv.starts_with(b"const ") {
        rv = &rv[6..];
    }

    rv
}

// Manipulate functions

pub fn find(v: &[u8], seq: &[u8]) -> Option<usize> {
    if seq.len() > v.len() {
        return None;
    }

    if v.starts_with(seq) || v == seq {
        return Some(0);
    }

    let mut seq_i = 0;
    for (i, byte) in v.iter().enumerate() {
        if *byte == seq[seq_i] {
            seq_i += 1;
        } else if *byte == seq[0] {
            seq_i = 1;
        } else {
            seq_i = 0;
        }

        if seq_i == seq.len() {
            return Some(i - (seq.len() - 1));
        }
    }

    None
}

pub fn find_all(v: &[u8], seq: &[u8]) -> Vec<usize> {
    if seq.len() > v.len() {
        return vec![];
    }

    let mut result = Vec::new();
    let mut seq_i = 0;
    for (i, byte) in v.iter().enumerate() {
        if *byte == seq[seq_i] {
            seq_i += 1;
        } else if *byte == seq[0] {
            seq_i = 1;
        } else {
            seq_i = 0;
        }

        if seq_i == seq.len() {
            result.push(i - (seq.len() - 1))
        }
    }

    result
}

pub fn split<'a>(mut v: &'a [u8], seq: &[u8]) -> Vec<&'a [u8]> {
    if seq.len() > v.len() {
        return vec![];
    }

    let mut result = Vec::new();

    while let Some(idx) = find(v, seq) {
        result.push(&v[..idx]);
        v = &v[idx + seq.len()..];
    }
    result.push(&v);

    result
}

pub fn contains_sequence(v: &[u8], seq: &[u8]) -> bool {
    if seq.len() > v.len() {
        return false;
    }

    let mut seq_i = 0;
    for byte in v {
        if *byte == seq[seq_i] {
            seq_i += 1;
        } else if *byte == seq[0] {
            seq_i = 1;
        } else {
            seq_i = 0;
        }

        if seq_i == seq.len() {
            return true;
        }
    }

    false
}
