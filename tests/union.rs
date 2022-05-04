use std::collections::BTreeSet;

use vector::test_helpers::{max_value, rand_array, FIRST_ARRAY_SIZE, SECOND_ARRAY_SIZE};
use vector::{FetchVec, UnionComplex, UnionVec, UnpackVec, Vector};

#[test]
fn union_complex_fetch() {
    let v1 = (0..30000).map(|i| (1 + i * 4)).collect::<Vec<_>>();
    let v2 = (0..200).map(|i| (2 + i * 4)).collect::<Vec<_>>();
    let v3 = (0..400).map(|i| (3 + i * 4)).collect::<Vec<_>>();
    let v4 = (0..100).map(|i| (4 + i * 4)).collect::<Vec<_>>();

    let mut expected = v1
        .iter()
        .chain(v2.iter())
        .chain(v3.iter())
        .chain(v4.iter())
        .map(|v| *v as i32)
        .collect::<Vec<_>>();

    expected.sort();

    let it = UnionComplex::new(vec![
        FetchVec::new(Vector::from(v4)),
        FetchVec::new(Vector::from(v3)),
        FetchVec::new(Vector::from(v2)),
        FetchVec::new(Vector::from(v1)),
    ]);

    let values = UnpackVec::new(it).collect::<Vec<_>>();
    assert_eq!(values, expected);
}

#[test]
fn union_vec_fetch() {
    let v1 = (0..300).map(|i| (1 + i * 4)).collect::<Vec<_>>();
    let v2 = (0..200).map(|i| (2 + i * 4)).collect::<Vec<_>>();
    let v3 = (0..400).map(|i| (3 + i * 4)).collect::<Vec<_>>();
    let v4 = (0..100).map(|i| (4 + i * 4)).collect::<Vec<_>>();

    let mut expected = v1
        .iter()
        .chain(v2.iter())
        .chain(v3.iter())
        .chain(v4.iter())
        .map(|v| *v as i32)
        .collect::<Vec<_>>();

    expected.sort();

    let it = UnionVec::new(
        UnionVec::new(
            UnionVec::new(
                FetchVec::new(Vector::from(v1)),
                FetchVec::new(Vector::from(v2)),
            ),
            FetchVec::new(Vector::from(v3)),
        ),
        FetchVec::new(Vector::from(v4)),
    );

    let values = UnpackVec::new(it).collect::<Vec<_>>();
    assert_eq!(values, expected);
}

#[test]
fn union_equals() {
    let a = rand_array(FIRST_ARRAY_SIZE, max_value(FIRST_ARRAY_SIZE));
    let b = rand_array(SECOND_ARRAY_SIZE, max_value(SECOND_ARRAY_SIZE));
    let expected = expected_union(&[&a, &b]);

    let v0 = Vector::from(a);
    let v1 = Vector::from(b);
    let uv = UnionVec::new(FetchVec::new(v0.clone()), FetchVec::new(v1.clone()));
    let uv_values = UnpackVec::new(uv).collect::<Vec<_>>();
    assert_eq!(uv_values, expected);

    let uc = UnionComplex::new(vec![FetchVec::new(v0), FetchVec::new(v1)]);
    let uc_values = UnpackVec::new(uc).collect::<Vec<_>>();
    assert_eq!(uc_values, expected);
}

#[test]
fn union_same_arrays() {
    let v = Vector::from([1, 10, 20, 30, 40, 50, 60, 70]);
    let uv = UnionVec::new(FetchVec::new(v.clone()), FetchVec::new(v.clone()));
    let uv_values = UnpackVec::new(uv).collect::<Vec<_>>();
    let uc = UnionComplex::new(vec![FetchVec::new(v.clone()), FetchVec::new(v)]);
    let uc_values = UnpackVec::new(uc).collect::<Vec<_>>();
    assert_eq!(uv_values, uc_values);
}

#[test]
fn union_empty() {
    let values = [1, 10, 20, 30, 40, 50, 60, 70];
    let expected = values.iter().map(|v| *v as i32).collect::<Vec<_>>();
    let v = Vector::from(values);
    let e = Vector::new();

    let fetch_vecs = vec![FetchVec::new(v.clone()), FetchVec::new(e.clone())];
    let it = UnionComplex::new(fetch_vecs);
    assert_eq!(UnpackVec::new(it).collect::<Vec<_>>(), expected);

    let fetch_vecs = vec![FetchVec::new(e.clone()), FetchVec::new(e.clone())];
    let it = UnionComplex::new(fetch_vecs);
    assert!(UnpackVec::new(it).collect::<Vec<_>>().is_empty());

    let it = UnionVec::new(FetchVec::new(v), FetchVec::new(e.clone()));
    assert_eq!(UnpackVec::new(it).collect::<Vec<_>>(), expected);

    let it = UnionVec::new(FetchVec::new(e.clone()), FetchVec::new(e));
    assert!(UnpackVec::new(it).collect::<Vec<_>>().is_empty());
}

#[test]
fn union_complex_reset() {
    let expected = [1, 10, 20, 30, 40, 50, 60, 70, 80, 90];
    let v0 = Vector::from([1, 10, 20, 30, 40, 50, 60, 70]);
    let v1 = Vector::from([80, 90]);
    let fetch_vecs = vec![FetchVec::new(v0), FetchVec::new(v1)];
    let mut it = UnpackVec::new(UnionComplex::new(fetch_vecs));

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

#[test]
fn union_vec_reset() {
    let expected = [1, 10, 20, 30, 40, 50, 60, 70, 80, 90];
    let v0 = Vector::from([1, 10, 20, 30, 40, 50, 60, 70]);
    let v1 = Vector::from([80, 90]);
    let mut it = UnpackVec::new(UnionVec::new(FetchVec::new(v0), FetchVec::new(v1)));

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

fn expected_union<T: AsRef<[u32]>>(arrays: &[T]) -> Vec<i32> {
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

    let mut intersection = sets[0].union(&sets[1]).copied().collect::<BTreeSet<_>>();

    for set in &sets[2..] {
        intersection = intersection.union(set).copied().collect::<BTreeSet<_>>();
    }

    intersection.into_iter().collect::<Vec<_>>()
}
