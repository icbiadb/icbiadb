
use std::collections::HashMap;

use crate::decl::types::{FieldMap, DeclarationMap, DeclarationRecord};
use crate::parser::globals::*;
use crate::utils::deserialize;
use crate::types::cursor::Cursor;


pub fn extract_length(v: &[u8]) -> (usize, usize, usize) {
	(deserialize::<usize>(&v[..USIZE_BS]),
		deserialize::<usize>(&v[USIZE_BS..USIZE_BS*2]),
		deserialize::<usize>(&v[USIZE_BS*2..USIZE_BS*3]))
}

pub fn seqs_find_all<'a>(v: &'a [u8], seq: &'a [u8]) -> Option<Vec<usize>> {
	assert_eq!(v[..3], decl::IDENT);
	let mut cursor = Cursor::new(&v);
	let mut idxs = Vec::new();
	let mut seq_i = 0;
	while cursor.position() < v.len() {
		if seq_i == seq.len() {
			cursor.jump(cursor.position() - seq.len());
			idxs.push(cursor.position());
			
			let (k_len, v_len) = get_decl_len(cursor.get(decl::IDENT_HEAD_BS));
			cursor.jump_forward(k_len + v_len + U64_BS*3);

			seq_i = 0;
		}

		if cursor.position() < v.len() && cursor.next() == seq[seq_i] {
			seq_i += 1;
		} else {
			seq_i = 0;
		}
	}

	if !idxs.is_empty() {
		return Some(idxs)
	} 

	None
}

pub fn get_decl_len<'a>(v: &'a [u8]) -> (usize, usize) {
	assert_eq!(&v[..3], decl::IDENT);
	(v[3] as usize, deserialize::<u16>(&v[4..4+U16_BS]) as usize)
}

pub fn extract_decl<'a>(v: &'a [u8], k_len: usize, v_len: usize) -> (&'a [u8], &'a [u8], u64, u64, u64) {
	assert_eq!(&v[..3], decl::IDENT);
	let mut cursor = Cursor::new(&v);
	cursor.jump(decl::IDENT_HEAD_BS);
	(
		&cursor.get(k_len),
		&cursor.get(v_len),
		deserialize(&cursor.get(U64_BS)),
		deserialize(&cursor.get(U64_BS)),
		deserialize(&cursor.get(U64_BS)),
	)
}

pub fn extract_decls(v: &[u8]) -> (HashMap<Vec<u8>, (u64, u64, u64)>, DeclarationMap) {
	assert_eq!(v[..3], decl::IDENT);
	let idxs = seqs_find_all(&v, &decl::IDENT).unwrap();

	let mut lu_map = HashMap::new();
	let mut decls = DeclarationMap::with_capacity(idxs.len());

	let mut cursor = Cursor::new(&v);
	for idx in idxs {
		cursor.jump(idx);
		let (k_len, v_len) = get_decl_len(cursor.peek(decl::IDENT_HEAD_BS));
		let (k, v, rstart, rlen, rcount) = extract_decl(cursor.get(decl::IDENT_HEAD_BS + k_len + v_len + U64_BS * 3), k_len, v_len);
		let fields: FieldMap = deserialize(&v);

		lu_map.insert(k.to_vec(), (rstart, rlen, rcount));
		decls.insert(k.to_vec(), fields);
	}

	(lu_map, decls)
}


pub mod records {
	use super::*;

	pub fn seqs_find_all<'a>(v: &'a [u8], seq: &'a [u8]) -> Option<Vec<usize>> {
		assert_eq!(v[..3], decl::records::IDENT);
		let mut cursor = Cursor::new(&v);
		let mut idxs = Vec::new();
		let mut seq_i = 0;
		while cursor.position() < v.len() {
			if seq_i == seq.len() {
				cursor.jump(cursor.position() - seq.len());
				// TODO
				// Fix cursor.seek(std::io::SeekFrom::Current(-(seq.len() as i64)));
				idxs.push(cursor.position());
				
				let r_len = get_record_len(cursor.get(decl::records::RECORD_IDENT_HEAD_BS)) as usize;
				cursor.jump_forward(r_len);

				seq_i = 0;
			}

			if cursor.position() < v.len() && cursor.next() == seq[seq_i] {
				seq_i += 1;
			} else {
				seq_i = 0;
			}
		}

		if !idxs.is_empty() {
			return Some(idxs)
		} 

		None
	}

	pub fn get_record_len<'a>(v: &'a [u8]) -> u32 {
		assert_eq!(&v[..3], decl::records::RECORD_IDENT);
		deserialize::<u32>(&v[3..3+U32_BS])
	}

	pub fn extract_single(v: &[u8], r_len: u32) -> DeclarationRecord {
		assert_eq!(&v[..3], decl::records::RECORD_IDENT);
		let mut cursor = Cursor::new(&v);
		cursor.jump(decl::records::RECORD_IDENT_HEAD_BS);
		deserialize(&cursor.get(r_len as usize))
	}

	pub fn extract(v: &[u8]) -> Vec<DeclarationRecord> {
		// Get index of all kv::IDENTs
		assert_eq!(v[..3], decl::records::IDENT);
		let idxs = seqs_find_all(&v, &decl::records::RECORD_IDENT).unwrap();

		/*
		// Check if last key is broken up
		let last_kv_range = &v[idxs[idxs.len()-1]..];
		if last_kv_range.len() <! kv::IDENT_HEAD_BS {
			let (k_len, t_len, v_len) = get_record_len(last_kv_range);
			if kv::IDENT_HEAD_BS + k_len + t_len + v_len != last_kv_range.len() {
				// Remove broken key
				idxs.pop();
			}
		}
		*/

		let mut cursor = Cursor::new(&v);
		let mut vec = Vec::with_capacity(idxs.len());

		// Retrieve kV
		for idx in idxs {
			cursor.jump(idx);
			let r_len = get_record_len(cursor.peek(decl::records::RECORD_IDENT_HEAD_BS));
			vec.push(extract_single(cursor.get(decl::records::RECORD_IDENT_HEAD_BS + r_len as usize), r_len))
		}
		
		vec
	}
}