use std::collections::BTreeSet;

use vector::test_helpers::{
    max_value, rand_array, FIRST_ARRAY_SIZE, INTERSECT_SIZE_LIST, SECOND_ARRAY_SIZE,
};
use vector::{FetchVec, IntersectComplex, IntersectVec, Iter, UnpackVec, Vector};

#[test]
fn intersect_vec() {
    let v1 = Vector::from([1, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 272]);
    let v2 = Vector::from([1, 10, 20, 30, 40, 50, 60, 260, 272]);
    let intersect = IntersectVec::new(FetchVec::new(&v1), FetchVec::new(&v2));
    let unpack = UnpackVec::new(intersect);
    assert_eq!(unpack.collect::<Vec<_>>(), [1, 10, 20, 30, 40, 50, 272]);
}

#[test]
fn intersect_complex() {
    let v1 = Vector::from([1, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 272]);
    let v2 = Vector::from([1, 10, 20, 30, 40, 50, 60, 260, 272]);
    let fetch_vecs = [FetchVec::new(&v1), FetchVec::new(&v2)];
    let intersect = IntersectComplex::new(fetch_vecs.into_iter());
    let unpack = UnpackVec::new(intersect);
    assert_eq!(unpack.collect::<Vec<_>>(), [1, 10, 20, 30, 40, 50, 272]);
}

#[test]
fn intersect_equal() {
    let a = rand_array(FIRST_ARRAY_SIZE, max_value(FIRST_ARRAY_SIZE));
    let b = rand_array(SECOND_ARRAY_SIZE, max_value(SECOND_ARRAY_SIZE));
    let expected = expected_intersection(&[&a, &b]);

    let v0 = Vector::from(a);
    let v1 = Vector::from(b);
    let iv = IntersectVec::new(FetchVec::new(&v0), FetchVec::new(&v1));
    let iv_values = UnpackVec::new(iv).collect::<Vec<_>>();
    assert_eq!(iv_values, expected);

    let ic = IntersectComplex::new([FetchVec::new(&v0), FetchVec::new(&v1)].into_iter());
    let ic_values = UnpackVec::new(ic).collect::<Vec<_>>();
    assert_eq!(ic_values, expected);
}

#[test]
fn intersect_many_equal() {
    let arrays = INTERSECT_SIZE_LIST
        .into_iter()
        .map(|s| rand_array(s, max_value(s)))
        .collect::<Vec<_>>();

    let expected = expected_intersection(&arrays);
    let vecs = arrays.into_iter().map(Vector::from).collect::<Vec<_>>();

    let iv = vecs[1..].iter().fold(
        Box::new(FetchVec::new(&vecs[0])) as Box<dyn Iter>,
        |acc, v| Box::new(IntersectVec::new(acc, FetchVec::new(v))),
    );

    let iv_values = UnpackVec::new(iv).collect::<Vec<_>>();
    assert_eq!(iv_values, expected);
    let ic = IntersectComplex::new(vecs.iter().map(FetchVec::new));
    let ic_values = UnpackVec::new(ic).collect::<Vec<_>>();
    assert_eq!(ic_values, expected);
}

#[test]
fn intersect_same_arrays() {
    let v = Vector::from([1, 10, 20, 30, 40, 50, 60, 70]);
    let iv = IntersectVec::new(FetchVec::new(&v), FetchVec::new(&v));
    let iv_values = UnpackVec::new(iv).collect::<Vec<_>>();
    let ic = IntersectComplex::new([FetchVec::new(&v), FetchVec::new(&v)].into_iter());
    let ic_values = UnpackVec::new(ic).collect::<Vec<_>>();
    assert_eq!(iv_values, ic_values);
}

#[test]
fn intersect_empty() {
    let v = Vector::from([1, 10, 20, 30, 40, 50, 60, 70]);
    let e = Vector::new();

    let fetch_vecs = [FetchVec::new(&v), FetchVec::new(&e)];
    let it = IntersectComplex::new(fetch_vecs.into_iter());
    assert!(UnpackVec::new(it).next().is_none());

    let fetch_vecs = [FetchVec::new(&e), FetchVec::new(&e)];
    let it = IntersectComplex::new(fetch_vecs.into_iter());
    assert!(UnpackVec::new(it).next().is_none());

    let it = IntersectVec::new(FetchVec::new(&v), FetchVec::new(&e));
    assert!(UnpackVec::new(it).next().is_none());

    let it = IntersectVec::new(FetchVec::new(&e), FetchVec::new(&e));
    assert!(UnpackVec::new(it).next().is_none());
}

#[test]
fn intersect_complex_reset() {
    let v0 = Vector::from([1, 10, 20, 30, 40, 50, 60, 70]);
    let v1 = Vector::from([10, 33, 50, 80, 90]);
    let fetch_vecs = [FetchVec::new(&v0), FetchVec::new(&v1)];
    let mut it = UnpackVec::new(IntersectComplex::new(fetch_vecs.into_iter()));
    assert_eq!(it.next(), Some(10));
    assert_eq!(it.next(), Some(50));
    assert!(it.next().is_none());
    it.reset();
    assert_eq!(it.next(), Some(10));
    assert_eq!(it.next(), Some(50));
    assert!(it.next().is_none());
}

#[test]
fn intersect_vec_reset() {
    let v0 = Vector::from([1, 10, 20, 30, 40, 50, 60, 70]);
    let v1 = Vector::from([10, 33, 50, 80, 90]);
    let mut it = UnpackVec::new(IntersectVec::new(FetchVec::new(&v0), FetchVec::new(&v1)));
    assert_eq!(it.next(), Some(10));
    assert_eq!(it.next(), Some(50));
    assert!(it.next().is_none());
    it.reset();
    assert_eq!(it.next(), Some(10));
    assert_eq!(it.next(), Some(50));
    assert!(it.next().is_none());
}

fn expected_intersection<T: AsRef<[u32]>>(arrays: &[T]) -> Vec<i32> {
    let sets = arrays
        .iter()
        .map(|array| {
            let mut set = BTreeSet::new();

            for value in array.as_ref().iter().copied() {
                set.insert(value as i32);
            }

            set
        })
        .collect::<Vec<_>>();

    let mut intersection = sets[0]
        .intersection(&sets[1])
        .copied()
        .collect::<BTreeSet<_>>();

    for set in &sets[2..] {
        intersection = intersection
            .intersection(set)
            .copied()
            .collect::<BTreeSet<_>>();
    }

    intersection.into_iter().collect::<Vec<_>>()
}
