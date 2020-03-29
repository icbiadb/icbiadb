use std::collections::HashMap;

use crate::parser::globals::*;
use crate::mem::{OwnedMemoryRecord};
use crate::utils::{Cursor, deserialize};


pub fn seqs_find_all<'a>(v: &'a [u8], seq: &'a [u8]) -> Vec<usize> {
	let mut cursor = Cursor::new(&v);
	let mut idxs = Vec::new();
	let mut seq_i = 0;
	while cursor.position() < v.len() {
		if seq_i == seq.len() {
			cursor.jump(cursor.position() - seq.len());
			idxs.push(cursor.position());

			let header = cursor.get(kv::IDENT_HEAD_BS);
			let (k_len, t_len, v_len) = get_ktv_len(&header);

			cursor.jump_forward(k_len + t_len + v_len);

			seq_i = 0;
		}

		if cursor.position() < v.len() && cursor.next() == seq[seq_i] {
			seq_i += 1;
		} else {
			seq_i = 0;
		}
	}

	idxs
}

pub fn get_keys_indexes(v: &[u8]) -> HashMap<usize, usize> {
	let idxs = seqs_find_all(&v, &kv::IDENT);
	let mut indexes = HashMap::with_capacity(idxs.len());

	let mut cursor = Cursor::new(&v);
	for idx in idxs {
		cursor.jump(idx);
		let (k_len, t_len, v_len) = get_ktv_len(cursor.peek(kv::IDENT_HEAD_BS));
		let (k, _, _) = extract_single(cursor.get(kv::IDENT_HEAD_BS + k_len + t_len + v_len), k_len, t_len, v_len);
		indexes.insert(k.iter().map(|b| *b as usize).sum::<usize>(), idx);
	}

	indexes
}

#[inline(never)]
pub fn get_ktv_len<'a>(v: &'a [u8]) -> (usize, usize, usize) {
	assert_eq!(&v[..3], kv::IDENT);
	(v[3] as usize,
		v[4] as usize,
		deserialize::<u32>(&v[5..5+V_LEN_BS]) as usize)
}

pub fn extract_single<'a>(v: &'a [u8], k_len: usize, t_len: usize, v_len: usize) -> OwnedMemoryRecord {
	assert_eq!(&v[..3], kv::IDENT);
	let mut cursor = Cursor::new(v);
	cursor.jump(kv::IDENT_HEAD_BS);
	(cursor.get(k_len).to_vec(), cursor.get(t_len).to_vec(), cursor.get(v_len).to_vec())
}

pub fn extract<'a>(v: &'a [u8]) -> Vec<OwnedMemoryRecord> {
	assert_eq!(v[..3], kv::IDENT);
	let idxs = seqs_find_all(&v, &kv::IDENT);

	let mut vec = Vec::with_capacity(idxs.len());

	let mut cursor = Cursor::new(&v);
	for idx in idxs {
		cursor.jump(idx);
		let (k_len, t_len, v_len) = get_ktv_len(cursor.peek(kv::IDENT_HEAD_BS));
		vec.push(extract_single(cursor.get(kv::IDENT_HEAD_BS + k_len + t_len + v_len), k_len, t_len, v_len))
	}

	vec
}
