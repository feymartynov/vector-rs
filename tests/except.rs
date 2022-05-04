use std::collections::BTreeSet;

use vector::test_helpers::{max_value, rand_array, FIRST_ARRAY_SIZE, SECOND_ARRAY_SIZE};
use vector::{ExceptVec, FetchVec, UnpackVec, Vector};

#[test]
fn except_vec() {
    let a: &[u32] = &[1, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 272];
    let b: &[u32] = &[1, 10, 20, 30, 40, 50, 60, 260, 272];
    let v1 = Vector::from(a.iter().copied());
    let v2 = Vector::from(b.iter().copied());
    let it = UnpackVec::new(ExceptVec::new(FetchVec::new(v1), FetchVec::new(v2)));
    assert_eq!(it.collect::<Vec<_>>(), [5, 15, 25, 35, 45]);
}

#[test]
fn except_equals() {
    let a = rand_array(FIRST_ARRAY_SIZE, max_value(FIRST_ARRAY_SIZE));
    let b = rand_array(SECOND_ARRAY_SIZE, max_value(SECOND_ARRAY_SIZE));

    let expected = {
        let mut values = BTreeSet::new();

        for value in a.iter().copied() {
            values.insert(value as i32);
        }

        for value in b.iter().copied() {
            values.remove(&(value as i32));
        }

        values.into_iter().collect::<Vec<_>>()
    };

    let ev = ExceptVec::new(
        FetchVec::new(Vector::from(a)),
        FetchVec::new(Vector::from(b)),
    );

    let ev_values = UnpackVec::new(ev).collect::<Vec<_>>();
    assert_eq!(ev_values, expected);
}

#[test]
fn except_same_arrays() {
    let a: &[u32] = &[1, 10, 20, 30, 40, 50, 60, 70];
    let v = Vector::from(a.iter().copied());
    let mut it = UnpackVec::new(ExceptVec::new(FetchVec::new(v.clone()), FetchVec::new(v)));
    assert!(it.next().is_none());
}

#[test]
fn except_empty() {
    let a: &[i32] = &[1, 10, 20, 30, 40, 50, 60, 70];
    let v = Vector::from(a.iter().copied().map(|x| x as u32));
    let e = Vector::new();

    let it = UnpackVec::new(ExceptVec::new(
        FetchVec::new(v.clone()),
        FetchVec::new(e.clone()),
    ));
    assert_eq!(it.collect::<Vec<_>>(), a);

    let mut it = UnpackVec::new(ExceptVec::new(FetchVec::new(e.clone()), FetchVec::new(v)));
    assert!(it.next().is_none());

    let mut it = UnpackVec::new(ExceptVec::new(FetchVec::new(e.clone()), FetchVec::new(e)));
    assert!(it.next().is_none());
}

#[test]
fn except_reset() {
    let a: &[u32] = &[1, 10, 20, 30, 40, 50, 60, 70];
    let b: &[u32] = &[10, 70, 98, 190];
    let v0 = Vector::from(a.iter().copied());
    let v1 = Vector::from(b.iter().copied());
    let expected = [1, 20, 30, 40, 50, 60];

    let mut it = UnpackVec::new(ExceptVec::new(FetchVec::new(v0), FetchVec::new(v1)));
    let mut values = Vec::new();

    while let Some(value) = it.next() {
        values.push(value);
    }

    assert_eq!(values, expected);
    values.clear();
    it.reset();

    while let Some(value) = it.next() {
        values.push(value);
    }

    assert_eq!(values, expected);
}
