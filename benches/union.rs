#[macro_use]
extern crate bencher;

use bencher::Bencher;
use vector::test_helpers::{rand_array, FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE, SECOND_ARRAY_SIZE};
use vector::{FetchVec, Iter, UnionComplex, UnionVec, Vector};

#[rustfmt::skip]
const SIZE_LIST: [usize; 135] = [FIRST_ARRAY_SIZE, 64316, SECOND_ARRAY_SIZE, 1313, 84, 173, 146, 974, 8369, 771, 555, 3983, 15693, 254, 1945, 11934, 1722, 6122, 1150, 893, 3, 234, 1670, 776, 2335, 1296, 150, 2215, 3518, 535, 1435, 561, 761, 1266, 278, 1347, 352, 1695, 8824, 609, 262, 998, 108, 832, 316, 818, 9, 233, 36, 24, 660, 214, 261, 903, 560, 34, 42, 9905, 25, 1, 1095, 258, 575, 861, 126, 535, 2025, 1064, 105, 1487, 485, 217, 345, 191, 1071, 220, 936, 96, 760, 305, 62, 2546, 79, 5, 65, 895, 38, 1926, 7, 77, 80, 27, 28, 295, 42, 284, 9569, 298, 39, 62, 78, 831, 56, 3, 66, 3, 18, 56, 497, 45, 548, 189, 63, 65, 12323, 72, 354, 497, 1006, 11511, 439, 2644, 95, 169, 514, 6, 159, 23, 63, 24, 18, 171, 8, 532, 36];

fn union_complex(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let mut it = UnionComplex::new(vec![FetchVec::new(v0), FetchVec::new(v1)]);

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn union_vec(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let mut it = UnionVec::new(FetchVec::new(v0), FetchVec::new(v1));

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn union_complex_many(bencher: &mut Bencher) {
    let fetch_vecs = SIZE_LIST
        .into_iter()
        .map(|s| FetchVec::new(Vector::from(rand_array(s, MAX_ARRAY_VALUE))))
        .collect::<Vec<_>>();

    let mut it = UnionComplex::new(fetch_vecs);

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn union_vec_many(bencher: &mut Bencher) {
    let mut fetch_vecs = SIZE_LIST
        .into_iter()
        .map(|s| FetchVec::new(Vector::from(rand_array(s, MAX_ARRAY_VALUE))))
        .collect::<Vec<_>>();

    let mut it = Box::new(UnionVec::new(
        fetch_vecs.pop().unwrap(),
        fetch_vecs.pop().unwrap(),
    )) as Box<dyn Iter>;

    for fetch_vec in fetch_vecs {
        it = Box::new(UnionVec::new(it, fetch_vec));
    }

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

benchmark_group!(
    benches,
    union_complex,
    union_vec,
    union_complex_many,
    union_vec_many
);

benchmark_main!(benches);
