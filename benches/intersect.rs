#[macro_use]
extern crate bencher;

use bencher::Bencher;
use vector::test_helpers::{
    max_value, rand_array, FIRST_ARRAY_SIZE, INTERSECT_SIZE_LIST, SECOND_ARRAY_SIZE,
};
use vector::{FetchVec, IntersectComplex, IntersectVec, Iter, UnpackVec, Vector};

fn intersect_vec(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, max_value(FIRST_ARRAY_SIZE)));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, max_value(SECOND_ARRAY_SIZE)));
    let mut it = IntersectVec::new(FetchVec::new(v0), FetchVec::new(v1));

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn intersect_vec_with_unpack(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, max_value(FIRST_ARRAY_SIZE)));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, max_value(SECOND_ARRAY_SIZE)));
    let mut it = UnpackVec::new(IntersectVec::new(FetchVec::new(v0), FetchVec::new(v1)));

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn intersect_complex(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, max_value(FIRST_ARRAY_SIZE)));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, max_value(SECOND_ARRAY_SIZE)));
    let mut it = IntersectComplex::new([FetchVec::new(v0), FetchVec::new(v1)].into_iter());

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn intersect_complex_with_unpack(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, max_value(FIRST_ARRAY_SIZE)));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, max_value(SECOND_ARRAY_SIZE)));
    let it = IntersectComplex::new([FetchVec::new(v0), FetchVec::new(v1)].into_iter());
    let mut it = UnpackVec::new(it);

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn intersect_vec_many(bencher: &mut Bencher) {
    let vecs = INTERSECT_SIZE_LIST
        .into_iter()
        .map(|s| Vector::from(rand_array(s, max_value(s))))
        .collect::<Vec<_>>();

    let mut it = vecs[1..].iter().fold(
        Box::new(FetchVec::new(vecs[0].clone())) as Box<dyn Iter>,
        |acc, v| Box::new(IntersectVec::new(acc, FetchVec::new(v.to_owned()))),
    );

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn intersect_vec_many_with_unpack(bencher: &mut Bencher) {
    let vecs = INTERSECT_SIZE_LIST
        .into_iter()
        .map(|s| Vector::from(rand_array(s, max_value(s))))
        .collect::<Vec<_>>();

    let it = vecs[1..].iter().fold(
        Box::new(FetchVec::new(vecs[0].clone())) as Box<dyn Iter>,
        |acc, v| Box::new(IntersectVec::new(acc, FetchVec::new(v.to_owned()))),
    );

    let mut it = UnpackVec::new(it);

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn intersect_complex_many(bencher: &mut Bencher) {
    let fetch_vecs = INTERSECT_SIZE_LIST
        .into_iter()
        .map(|s| FetchVec::new(Vector::from(rand_array(s, max_value(s)))));

    let mut it = IntersectComplex::new(fetch_vecs);

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn intersect_complex_many_with_unpack(bencher: &mut Bencher) {
    let fetch_vecs = INTERSECT_SIZE_LIST
        .into_iter()
        .map(|s| FetchVec::new(Vector::from(rand_array(s, max_value(s)))));

    let mut it = UnpackVec::new(IntersectComplex::new(fetch_vecs));

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

benchmark_group!(
    benches,
    intersect_vec,
    intersect_vec_with_unpack,
    intersect_complex,
    intersect_complex_with_unpack,
    intersect_vec_many,
    intersect_vec_many_with_unpack,
    intersect_complex_many,
    intersect_complex_many_with_unpack
);

benchmark_main!(benches);
