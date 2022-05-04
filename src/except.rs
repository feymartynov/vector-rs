use std::cmp::Ordering;

use super::{Iter, IterItem};

#[derive(Debug)]
struct Item<T: Iter> {
    vec: T,
    iter_item: Option<IterItem>,
}

impl<T: Iter> Item<T> {
    fn new(vec: T) -> Self {
        Self {
            vec,
            iter_item: None,
        }
    }
}

#[derive(Debug)]
pub struct ExceptVec<A: Iter, B: Iter> {
    it: A,
    except: Item<B>,
}

impl<A: Iter, B: Iter> ExceptVec<A, B> {
    pub fn new(it: A, except: B) -> Self {
        let mut except_vec = Self {
            it,
            except: Item::new(except),
        };

        except_vec.reset();
        except_vec
    }
}

impl<A: Iter, B: Iter> Iterator for ExceptVec<A, B> {
    type Item = IterItem;

    fn next(&mut self) -> Option<Self::Item> {
        for mut item in self.it.by_ref() {
            if self.except.iter_item.is_none() {
                return Some(item);
            }

            loop {
                let base = self.except.iter_item.map(|i| i.base).unwrap_or(0);

                match item.base.cmp(&base) {
                    Ordering::Less => return Some(item),
                    Ordering::Greater => {
                        self.except.iter_item = self.except.vec.next();

                        if self.except.iter_item.is_none() {
                            return Some(item);
                        }
                    }
                    Ordering::Equal => {
                        match self.except.iter_item {
                            Some(except_item) => item.bitset ^= item.bitset & except_item.bitset,
                            None => unreachable!(),
                        }

                        match item.bitset.is_zero() {
                            true => break,
                            false => return Some(item),
                        }
                    }
                }
            }
        }

        None
    }
}

impl<A: Iter, B: Iter> Iter for ExceptVec<A, B> {
    fn reset(&mut self) {
        self.it.reset();
        self.except.vec.reset();
        self.except.iter_item = self.except.vec.next();
    }
}
