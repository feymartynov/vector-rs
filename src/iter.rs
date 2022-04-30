use crate::{Bitset, Vector, BITS_SIZE};

#[derive(Debug)]
pub struct IterItem {
    pub base: u16,
    pub bitset: Bitset,
}

pub trait Iter: Iterator<Item = IterItem> {
    fn reset(&mut self);
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct FetchVec {
    position: usize,
    offset: usize,
    vector: Vector,
}

impl FetchVec {
    pub fn new(vector: Vector) -> Self {
        Self {
            position: 0,
            offset: 0,
            vector,
        }
    }
}

impl Iterator for FetchVec {
    type Item = IterItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.vector.head.len() {
            let head = &self.vector.head[self.position];
            let base = head.base;
            let n = self.offset + head.size as usize + 1;
            let mut bitset = Bitset::default();
            bitset.bytes_to_bits(&self.vector.data[self.offset..n]);
            self.position += 1;
            self.offset = n;
            Some(IterItem { base, bitset })
        } else {
            None
        }
    }
}

impl Iter for FetchVec {
    fn reset(&mut self) {
        self.position = 0;
        self.offset = 0;
    }
}

///////////////////////////////////////////////////////////////////////////////

// Doesn't impl Iter on purpose since it's always on the top level and yields integers.
#[derive(Debug)]
pub struct UnpackVec<I: Iter> {
    it: I,
    base: u16,
    i: u16,
    values: Vec<u8>,
    len: usize,
}

impl<I: Iter> UnpackVec<I> {
    pub fn new(it: I) -> Self {
        Self {
            it,
            base: 0,
            i: 0,
            values: vec![0; BITS_SIZE],
            len: 0,
        }
    }

    pub fn reset(&mut self) {
        self.it.reset();
        self.base = 0;
        self.i = 0;
        self.values.fill(0);
        self.len = 0;
    }
}

impl<I: Iter> Iterator for UnpackVec<I> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i as usize >= self.len {
            if let Some(item) = self.it.next() {
                self.base = item.base;
                self.len = item.bitset.bits_to_bytes(&mut self.values);
                self.i = 0;
            } else {
                return None;
            }
        }

        let v = (self.base as i32) << 8 | self.values[self.i as usize] as i32;
        self.i += 1;
        Some(v)
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct Empty {}

impl Empty {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Iterator for Empty {
    type Item = IterItem;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Iter for Empty {
    fn reset(&mut self) {}
}
