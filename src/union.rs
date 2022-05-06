use std::cmp::Ordering;

use crate::{Bitset, Empty, FetchVec, Iter, IterItem, Vector};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct UnionVecItem<T: Iter> {
    vec: T,
    iter_item: Option<IterItem>,
}

impl<T: Iter> UnionVecItem<T> {
    fn new(vec: T) -> Self {
        Self {
            vec,
            iter_item: None,
        }
    }
}

#[derive(Debug)]
pub struct UnionVec<A: Iter, B: Iter> {
    a: UnionVecItem<A>,
    b: UnionVecItem<B>,
}

impl<A: Iter, B: Iter> UnionVec<A, B> {
    pub fn new(a: A, b: B) -> Self {
        let mut vec = Self {
            a: UnionVecItem::new(a),
            b: UnionVecItem::new(b),
        };

        vec.reset();
        vec
    }

    fn next_a(&mut self) -> Option<IterItem> {
        let maybe_item = self.a.iter_item;
        self.a.iter_item = self.a.vec.next();
        maybe_item
    }

    fn next_b(&mut self) -> Option<IterItem> {
        let maybe_item = self.b.iter_item;
        self.b.iter_item = self.b.vec.next();
        maybe_item
    }

    fn next_a_or_b(&mut self) -> Option<IterItem> {
        let maybe_item = match (self.a.iter_item, self.b.iter_item) {
            (Some(a), Some(b)) => {
                let mut item = a;
                item.bitset |= b.bitset;
                Some(item)
            }
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        self.a.iter_item = self.a.vec.next();
        self.b.iter_item = self.b.vec.next();
        maybe_item
    }
}

impl<A: Iter, B: Iter> Iterator for UnionVec<A, B> {
    type Item = IterItem;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.iter_item, self.b.iter_item) {
            (Some(a), Some(b)) => match a.base.cmp(&b.base) {
                Ordering::Less => self.next_a(),
                Ordering::Greater => self.next_b(),
                Ordering::Equal => self.next_a_or_b(),
            },
            (Some(_), None) => self.next_a(),
            (None, Some(_)) => self.next_b(),
            (None, None) => None,
        }
    }
}

impl<A: Iter, B: Iter> Iter for UnionVec<A, B> {
    fn reset(&mut self) {
        self.a.vec.reset();
        self.b.vec.reset();
        self.a.iter_item = self.a.vec.next();
        self.b.iter_item = self.b.vec.next();
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct UnionComplexItem {
    origin_idx: usize,
    iter_item: IterItem,
}

#[derive(Debug)]
pub struct UnionComplex<T: Iter> {
    origin: Vec<T>,
    items: Vec<UnionComplexItem>,
    current_base: u16,
}

impl<T: Iter> UnionComplex<T> {
    pub fn new(origin: Vec<T>) -> Self {
        let items = Vec::with_capacity(origin.len());

        let mut complex = Self {
            origin,
            items,
            current_base: 0,
        };

        complex.reset();
        complex
    }
}

impl<T: Iter> Iterator for UnionComplex<T> {
    type Item = IterItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return None;
        }

        let mut bitset = Bitset::default();
        let mut min = u16::MAX;
        let mut i = 0;

        // TODO: Rewrite with `Vec::drain_filter` when stabilized.
        while i < self.items.len() {
            let item = unsafe { self.items.get_unchecked_mut(i) };

            if item.iter_item.base == self.current_base {
                bitset |= item.iter_item.bitset;
                let origin = unsafe { self.origin.get_unchecked_mut(item.origin_idx) };

                match origin.next() {
                    Some(next) => {
                        item.iter_item.base = next.base;
                        item.iter_item.bitset = next.bitset;

                        if item.iter_item.base < min {
                            min = item.iter_item.base;
                        }
                    }
                    None => {
                        self.items.remove(i);
                        continue;
                    }
                }
            } else if item.iter_item.base < min {
                min = item.iter_item.base;
            }

            i += 1;
        }

        let base = self.current_base;
        self.current_base = min;
        Some(IterItem::new(base, bitset))
    }
}

impl<T: Iter> Iter for UnionComplex<T> {
    fn reset(&mut self) {
        self.items.clear();
        let mut min = u16::MAX;

        for (origin_idx, iter) in self.origin.iter_mut().enumerate() {
            iter.reset();

            if let Some(iter_item) = iter.next() {
                if iter_item.base < min {
                    min = iter_item.base;
                }

                self.items.push(UnionComplexItem {
                    origin_idx,
                    iter_item,
                });
            }
        }

        self.current_base = min;
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Union<T: Iter> {
    Empty(Empty),
    Single(T),
    Pair(UnionVec<T, T>),
    Complex(UnionComplex<T>),
}

impl<T: Iter> Iterator for Union<T> {
    type Item = IterItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty(empty) => empty.next(),
            Self::Single(iter) => iter.next(),
            Self::Pair(vec) => vec.next(),
            Self::Complex(complex) => complex.next(),
        }
    }
}

impl<T: Iter> Iter for Union<T> {
    fn reset(&mut self) {
        match self {
            Self::Empty(empty) => empty.reset(),
            Self::Single(iter) => iter.reset(),
            Self::Pair(vec) => vec.reset(),
            Self::Complex(complex) => complex.reset(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct Builder {
    source: Vec<FetchVec>,
}

impl Builder {
    pub fn new() -> Self {
        Self { source: Vec::new() }
    }

    pub fn add(&mut self, vec: Vector) -> &mut Self {
        self.source.push(FetchVec::new(vec));
        self
    }

    pub fn build(self) -> Union<FetchVec> {
        match self.source.len() {
            0 => Union::Empty(Empty::new()),
            1 => Union::Single(self.source.into_iter().next().unwrap()),
            2 => {
                let mut source = self.source.into_iter();
                let a = source.next().unwrap();
                let b = source.next().unwrap();
                Union::Pair(UnionVec::new(a, b))
            }
            _ => Union::Complex(UnionComplex::new(self.source)),
        }
    }
}
