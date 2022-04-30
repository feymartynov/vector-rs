use vector::{Empty, FetchVec, Iter, UnpackVec, Vector};

#[test]
fn fetch_vec_solid() {
    let unpack_vec = UnpackVec::new(FetchVec::new(Vector::from(1..255)));
    let values = unpack_vec.collect::<Vec<_>>();
    assert_eq!(values, (1..255).into_iter().collect::<Vec<_>>());
}

#[test]
fn fetch_vec_with_duplicates() {
    let mut vector = Vector::new();

    for i in 0..6 {
        vector.add(i as u32);
        vector.add(i as u32);
    }

    let values = UnpackVec::new(FetchVec::new(vector)).collect::<Vec<_>>();
    assert_eq!(values, [0, 1, 2, 3, 4, 5]);
}

#[test]
fn fetch_unpack_vec() {
    let expected = [8i32, 320, 1536, 1544, 266752, 266800, 791088];
    let vector = Vector::from([8u32, 320, 1536, 1544, 266752, 266800, 791088]);
    let mut unpack = UnpackVec::new(FetchVec::new(vector));
    let mut values = Vec::new();

    for value in &mut unpack {
        values.push(value);
    }

    assert_eq!(values, expected);
    unpack.reset();
    assert_eq!(unpack.collect::<Vec<_>>(), expected);
}

#[test]
fn empty() {
    let mut empty = Empty::new();
    assert!(empty.next().is_none());
    empty.reset();
    assert!(empty.next().is_none());
}
