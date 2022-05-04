#[macro_use]
extern crate bencher;

use bencher::Bencher;
use vector::test_helpers::{rand_array, FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE, SECOND_ARRAY_SIZE};
use vector::{ExceptVec, FetchVec, Iter, UnpackVec, Vector};

fn except_vec(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let mut it = ExceptVec::new(FetchVec::new(v0), FetchVec::new(v1));

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

fn except_vec_with_unpack(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let mut it = UnpackVec::new(ExceptVec::new(FetchVec::new(v0), FetchVec::new(v1)));

    bencher.iter(|| {
        while let Some(_) = it.next() {}
        it.reset();
    })
}

benchmark_group!(benches, except_vec, except_vec_with_unpack);
benchmark_main!(benches);