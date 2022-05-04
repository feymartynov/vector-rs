use std::cmp::Ordering;

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Header {
    pub(crate) base: u16,
    pub(crate) size: u8,
}

impl Header {
    fn new(base: u16) -> Self {
        Self { base, size: 0 }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Vector {
    pub(crate) head: Vec<Header>,
    pub(crate) data: Vec<u8>,
    pub(crate) last: u32,
}

impl Vector {
    pub fn new() -> Self {
        Self {
            head: Vec::new(),
            data: Vec::new(),
            last: u32::MAX,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn reset(&mut self) {
        self.head.clear();
        self.data.clear();
        self.last = u32::MAX;
    }

    pub fn add(&mut self, n: u32) {
        if n == self.last {
            return;
        }

        let base = (n >> 8) as u16;

        if let Some(last) = self.head.last_mut() {
            match last.base.cmp(&base) {
                Ordering::Equal => last.size += 1,
                Ordering::Less => self.head.push(Header::new(base)),
                Ordering::Greater => panic!("Wrong data"),
            }
        } else {
            self.head.push(Header::new(base));
        }

        self.data.push(n as u8);
        self.last = n;
    }

    pub fn capacity(&self) -> usize {
        self.head.capacity() * 4 + self.data.capacity()
    }

    pub fn len(&self) -> usize {
        self.head.len() * 4 + self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }
}

impl<T: IntoIterator<Item = u32>> From<T> for Vector {
    fn from(values: T) -> Self {
        let mut vector = Self::new();

        for value in values.into_iter() {
            vector.add(value);
        }

        vector
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug)]
pub struct IterItem<'a> {
    pub base: u16,
    pub size: u8,
    pub data: &'a [u8],
}

#[derive(Debug)]
pub struct Iter<'a> {
    vector: &'a Vector,
    position: usize,
    offset: usize,
}

impl<'a> Iter<'a> {
    fn new(vector: &'a Vector) -> Self {
        Self {
            vector,
            position: 0,
            offset: 0,
        }
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.offset = 0;
    }

    pub fn has_next(&self) -> bool {
        self.position + 1 < self.vector.head.len()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = IterItem<'a>;

    fn next(&mut self) -> Option<IterItem<'a>> {
        if self.has_next() {
            self.offset += self.vector.head[self.position].size as usize;
            self.position += 1;
            let head = &self.vector.head[self.position];

            Some(IterItem {
                base: head.base,
                size: head.size,
                data: &self.vector.data[self.offset..(self.offset + head.size as usize)],
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.vector.head.len() - self.position - 1;
        (len, Some(len))
    }
}
