#[macro_use]
extern crate bencher;

use bencher::Bencher;
use vector::test_helpers::{max_value, rand_array, FIRST_ARRAY_SIZE, SECOND_ARRAY_SIZE};
use vector::{ExceptVec, FetchVec, Iter, UnpackVec, Vector};

fn except_vec(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, max_value(FIRST_ARRAY_SIZE)));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, max_value(SECOND_ARRAY_SIZE)));
    let mut it = ExceptVec::new(FetchVec::new(&v0), FetchVec::new(&v1));

    bencher.iter(|| {
        for _ in it.by_ref() {}
        it.reset();
    })
}

fn except_vec_with_unpack(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, max_value(FIRST_ARRAY_SIZE)));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, max_value(SECOND_ARRAY_SIZE)));
    let mut it = UnpackVec::new(ExceptVec::new(FetchVec::new(&v0), FetchVec::new(&v1)));

    bencher.iter(|| {
        for _ in it.by_ref() {}
        it.reset();
    })
}

benchmark_group!(benches, except_vec, except_vec_with_unpack);
benchmark_main!(benches);
