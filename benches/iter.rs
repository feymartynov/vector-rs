#[macro_use]
extern crate bencher;

use bencher::Bencher;
use vector::{test_helpers::rand_array, FetchVec, UnpackVec, Vector};

const ARRAY_LEN: usize = 128 * 1024;
const MAX_ARRAY_VALUE: u32 = 16 * 1024 * 1024;

fn fetch_unpack_vec(bencher: &mut Bencher) {
    let a = rand_array(ARRAY_LEN, MAX_ARRAY_VALUE);
    let mut unpack = UnpackVec::new(FetchVec::new(Vector::from(a)));

    bencher.iter(|| {
        for _ in &mut unpack {}
        unpack.reset();
    })
}

benchmark_group!(benches, fetch_unpack_vec);
benchmark_main!(benches);
