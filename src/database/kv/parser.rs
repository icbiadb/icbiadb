use crate::byte_size::globals::*;
use crate::storage::KvInterface;
use crate::types::cursor::Cursor;
use crate::types::{BvObject, BvString};
use crate::utils::deserialize;

pub fn seqs_find_all(v: &[u8], seq: &[u8]) -> Vec<usize> {
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

#[inline(never)]
pub fn get_ktv_len(v: &[u8]) -> (usize, usize, usize) {
    assert_eq!(&v[..3], kv::IDENT);
    (
        v[3] as usize,
        v[4] as usize,
        deserialize::<u32>(&v[5..5 + V_LEN_BS]) as usize,
    )
}

pub fn extract_single(v: &[u8], k_len: usize, t_len: usize, v_len: usize) -> (Vec<u8>, BvObject) {
    assert_eq!(&v[..3], kv::IDENT);
    let mut cursor = Cursor::new(v);
    cursor.jump(kv::IDENT_HEAD_BS);
    (
        cursor.get(k_len).into(),
        (cursor.get(t_len), cursor.get(v_len)).into(),
    )
}

pub fn extract_records<KV: KvInterface<Key = BvString, Value = BvObject>>(v: &[u8]) -> KV {
    assert_eq!(v[..3], kv::IDENT);
    let idxs = seqs_find_all(&v, &kv::IDENT);

    //let mut vec = std::collections::HashMap::with_capacity(idxs.len());
    let mut storage = KV::default();

    let mut cursor = Cursor::new(&v);
    for idx in idxs {
        cursor.jump(idx);
        let (k_len, t_len, v_len) = get_ktv_len(cursor.peek(kv::IDENT_HEAD_BS));
        let (k, v) = extract_single(
            cursor.get(kv::IDENT_HEAD_BS + k_len + t_len + v_len),
            k_len,
            t_len,
            v_len,
        );
        storage.insert(BvString::from(k), v);
    }

    storage
}
