use std::collections::HashMap;

use super::types::{FieldMap, TableMap, TableRow};
use crate::byte_size::globals::*;
use crate::types::cursor::Cursor;
use crate::utils::deserialize;

pub fn extract_length(v: &[u8]) -> (usize, usize, usize) {
    (
        deserialize::<usize>(&v[..USIZE_BS]),
        deserialize::<usize>(&v[USIZE_BS..USIZE_BS * 2]),
        deserialize::<usize>(&v[USIZE_BS * 2..USIZE_BS * 3]),
    )
}

pub fn seqs_find_all<'a>(v: &'a [u8], seq: &'a [u8]) -> Option<Vec<usize>> {
    assert_eq!(v[..3], table::IDENT);
    let mut cursor = Cursor::new(&v);
    let mut idxs = Vec::new();
    let mut seq_i = 0;
    while cursor.position() < v.len() {
        if seq_i == seq.len() {
            cursor.jump(cursor.position() - seq.len());
            idxs.push(cursor.position());

            let (k_len, v_len) = get_table_len(cursor.get(table::IDENT_HEAD_BS));
            cursor.jump_forward(k_len + v_len + U64_BS * 3);

            seq_i = 0;
        }

        if cursor.position() < v.len() && cursor.next() == seq[seq_i] {
            seq_i += 1;
        } else {
            seq_i = 0;
        }
    }

    if !idxs.is_empty() {
        return Some(idxs);
    }

    None
}

pub fn get_table_len(v: &[u8]) -> (usize, usize) {
    assert_eq!(&v[..3], table::IDENT);
    (
        v[3] as usize,
        deserialize::<u16>(&v[4..4 + U16_BS]) as usize,
    )
}

pub fn extract_table(v: &[u8], k_len: usize, v_len: usize) -> (&[u8], &[u8], u64, u64, u64) {
    assert_eq!(&v[..3], table::IDENT);
    let mut cursor = Cursor::new(&v);
    cursor.jump(table::IDENT_HEAD_BS);
    (
        &cursor.get(k_len),
        &cursor.get(v_len),
        deserialize(&cursor.get(U64_BS)),
        deserialize(&cursor.get(U64_BS)),
        deserialize(&cursor.get(U64_BS)),
    )
}

pub fn extract_tables(v: &[u8]) -> (HashMap<Vec<u8>, (u64, u64, u64)>, TableMap) {
    assert_eq!(v[..3], table::IDENT);
    let idxs = seqs_find_all(&v, &table::IDENT).unwrap();

    let mut lu_map = HashMap::new();
    let mut tables = TableMap::with_capacity(idxs.len());

    let mut cursor = Cursor::new(&v);
    for idx in idxs {
        cursor.jump(idx);
        let (k_len, v_len) = get_table_len(cursor.peek(table::IDENT_HEAD_BS));
        let (k, v, rstart, rlen, rcount) = extract_table(
            cursor.get(table::IDENT_HEAD_BS + k_len + v_len + U64_BS * 3),
            k_len,
            v_len,
        );
        let fields: FieldMap = deserialize(&v);

        lu_map.insert(k.to_vec(), (rstart, rlen, rcount));
        tables.insert(k.to_vec(), fields);
    }

    (lu_map, tables)
}

pub mod rows {
    use super::*;

    pub fn seqs_find_all<'a>(v: &'a [u8], seq: &'a [u8]) -> Option<Vec<usize>> {
        assert_eq!(v[..3], table::rows::IDENT);
        let mut cursor = Cursor::new(&v);
        let mut idxs = Vec::new();
        let mut seq_i = 0;
        while cursor.position() < v.len() {
            if seq_i == seq.len() {
                cursor.jump(cursor.position() - seq.len());
                // TODO
                // Fix cursor.seek(std::io::SeekFrom::Current(-(seq.len() as i64)));
                idxs.push(cursor.position());

                let r_len = get_record_len(cursor.get(table::rows::RECORD_IDENT_HEAD_BS)) as usize;
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
            return Some(idxs);
        }

        None
    }

    pub fn get_record_len(v: &[u8]) -> u32 {
        assert_eq!(&v[..3], table::rows::RECORD_IDENT);
        deserialize::<u32>(&v[3..3 + U32_BS])
    }

    pub fn extract_single(v: &[u8], r_len: u32) -> TableRow {
        assert_eq!(&v[..3], table::rows::RECORD_IDENT);
        let mut cursor = Cursor::new(&v);
        cursor.jump(table::rows::RECORD_IDENT_HEAD_BS);
        deserialize(&cursor.get(r_len as usize))
    }

    pub fn extract_rows(v: &[u8]) -> Vec<TableRow> {
        // Get index of all kv::IDENTs
        assert_eq!(v[..3], table::rows::IDENT);
        let idxs = seqs_find_all(&v, &table::rows::RECORD_IDENT).unwrap();

        let mut cursor = Cursor::new(&v);
        let mut vec = Vec::with_capacity(idxs.len());

        for idx in idxs {
            cursor.jump(idx);
            let r_len = get_record_len(cursor.peek(table::rows::RECORD_IDENT_HEAD_BS));
            vec.push(extract_single(
                cursor.get(table::rows::RECORD_IDENT_HEAD_BS + r_len as usize),
                r_len,
            ))
        }

        vec
    }
}
