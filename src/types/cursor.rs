use std::io::SeekFrom;

/// Helper function for byte vector traversal

pub struct Cursor<'a> {
    inner: &'a [u8],
    cursor: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(b: &'a [u8]) -> Self {
        Cursor {
            inner: b,
            cursor: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.cursor
    }

    pub fn set_position(&mut self, pos: usize) {
        self.cursor = pos;
    }

    pub fn remaining(&mut self) -> &'a [u8] {
        let r = &self.inner[self.cursor..];
        self.cursor += r.len();
        r
    }

    pub fn remaining_len(&self) -> usize {
        self.inner[self.cursor..].len()
    }

    pub fn next(&mut self) -> u8 {
        let r = self.inner[self.cursor];
        self.cursor += 1;
        r
    }

    pub fn peek(&mut self, len: usize) -> &'a [u8] {
        &self.inner[self.cursor..self.cursor + len]
    }

    pub fn get(&mut self, len: usize) -> &'a [u8] {
        let r = &self.inner[self.cursor..self.cursor + len];
        self.cursor += len;
        r
    }

    pub fn jump(&mut self, pos: usize) {
        self.cursor = pos;
    }

    pub fn jump_forward(&mut self, pos: usize) {
        self.cursor += pos;
    }

    pub fn seek(&mut self, s: SeekFrom) {
        match s {
            SeekFrom::Start(p) => self.cursor = p as usize,
            SeekFrom::Current(p) => self.cursor += p as usize,
            SeekFrom::End(p) => self.cursor = self.inner.len() - p as usize,
        }
    }
}
