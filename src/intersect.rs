use std::cmp::Ordering;

use crate::{Bitset, Iter, IterItem};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct IntersectVec<A: Iter, B: Iter> {
    a: A,
    b: B,
}

impl<A: Iter, B: Iter> IntersectVec<A, B> {
    pub fn new(a: A, b: B) -> Self {
        let mut this = Self { a, b };
        this.reset();
        this
    }
}

impl<A: Iter, B: Iter> Iterator for IntersectVec<A, B> {
    type Item = IterItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut a = self.a.next()?;
        let mut b = self.b.next()?;

        loop {
            match a.base.cmp(&b.base) {
                Ordering::Less => a = self.a.next()?,
                Ordering::Greater => b = self.b.next()?,
                Ordering::Equal => {
                    a.bitset &= b.bitset;

                    if a.bitset.is_zero() {
                        a = self.a.next()?;
                        b = self.b.next()?;
                    } else {
                        break;
                    }
                }
            }
        }

        Some(a)
    }
}

impl<A: Iter, B: Iter> Iter for IntersectVec<A, B> {
    fn reset(&mut self) {
        self.a.reset();
        self.b.reset();
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Item<T: Iter> {
    vec: T,
    base: u16,
    bitset: Bitset,
}

impl<T: Iter> Item<T> {
    fn new(vec: T) -> Self {
        Self {
            vec,
            base: 0,
            bitset: Bitset::default(),
        }
    }

    fn next(&mut self) -> bool {
        match self.vec.next() {
            Some(next) => {
                self.base = next.base;
                self.bitset = next.bitset;
                true
            }
            None => {
                self.base = 0;
                self.bitset = Bitset::default();
                false
            }
        }
    }
}

#[derive(Debug)]
pub struct IntersectComplex<T: Iter> {
    items: Vec<Item<T>>,
}

impl<T: Iter> IntersectComplex<T> {
    pub fn new(source: impl Iterator<Item = T>) -> Self {
        let items = source.map(Item::new).collect::<Vec<_>>();
        Self { items }
    }
}

impl<T: Iter> Iterator for IntersectComplex<T> {
    type Item = IterItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut a;
        let mut b;

        loop {
            a = self.items[0].vec.next()?;
            b = self.items[1].vec.next()?;
            let mut ok;

            loop {
                match a.base.cmp(&b.base) {
                    Ordering::Less => a = self.items[0].vec.next()?,
                    Ordering::Greater => b = self.items[1].vec.next()?,
                    Ordering::Equal => {
                        a.bitset &= b.bitset;
                        ok = !a.bitset.is_zero();
                        break;
                    }
                }
            }

            if !ok {
                continue;
            }

            for item in &mut self.items[2..] {
                while item.base < a.base {
                    if !item.next() {
                        return None;
                    }
                }

                if item.base > a.base {
                    ok = false;
                    break;
                }

                a.bitset &= item.bitset;
                ok = !a.bitset.is_zero();

                if !ok {
                    break;
                }
            }

            if ok {
                return Some(a);
            }
        }
    }
}

impl<T: Iter> Iter for IntersectComplex<T> {
    fn reset(&mut self) {
        for (i, item) in self.items.iter_mut().enumerate() {
            item.vec.reset();

            if i > 1 {
                let _result = item.next();
            }
        }
    }
}
