#[macro_use]
extern crate bencher;

use bencher::Bencher;
use vector::test_helpers::{rand_array, FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE};
use vector::{FetchVec, UnpackVec, Vector};

fn fetch_unpack_vec(bencher: &mut Bencher) {
    let v = Vector::from(rand_array(FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let mut unpack = UnpackVec::new(FetchVec::new(&v));

    bencher.iter(|| {
        for _ in &mut unpack {}
        unpack.reset();
    })
}

benchmark_group!(benches, fetch_unpack_vec);
benchmark_main!(benches);
