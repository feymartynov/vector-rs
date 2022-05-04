#[macro_use]
extern crate bencher;

use std::cmp::Ordering;

use bencher::Bencher;
use vector::test_helpers::{rand_array, FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE, SECOND_ARRAY_SIZE};
use vector::{Bitset, Vector};

fn intersect_array(bencher: &mut Bencher) {
    let a = rand_array(FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE);
    let b = rand_array(SECOND_ARRAY_SIZE, MAX_ARRAY_VALUE);

    bencher.iter(|| {
        let mut a_it = a.iter();
        let mut b_it = b.iter();
        let mut a_item = a_it.next();
        let mut b_item = b_it.next();

        while let (Some(a_value), Some(b_value)) = (a_item, b_item) {
            match a_value.cmp(&b_value) {
                Ordering::Less => a_item = a_it.next(),
                Ordering::Greater => b_item = b_it.next(),
                Ordering::Equal => {
                    a_item = a_it.next();
                    b_item = b_it.next();
                }
            }
        }
    });
}

fn intersect_vector(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, MAX_ARRAY_VALUE));

    bencher.iter(|| {
        let mut v0_it = v0.iter();
        let mut v1_it = v1.iter();
        let mut v0_item = v0_it.next();
        let mut v1_item = v1_it.next();

        while let (Some(v0_value), Some(v1_value)) = (v0_item, v1_item) {
            match v0_value.base.cmp(&v1_value.base) {
                Ordering::Less => v0_item = v0_it.next(),
                Ordering::Greater => v1_item = v1_it.next(),
                Ordering::Equal => {
                    let mut r0 = Bitset::default();
                    r0.bytes_to_bits(&v0_value.data);

                    let mut r1 = Bitset::default();
                    r1.bytes_to_bits(&v1_value.data);

                    r0 &= r1;
                    v0_item = v0_it.next();
                    v1_item = v1_it.next();
                }
            }
        }
    });
}

fn union_array(bencher: &mut Bencher) {
    let a = rand_array(FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE);
    let b = rand_array(SECOND_ARRAY_SIZE, MAX_ARRAY_VALUE);

    bencher.iter(|| {
        let mut a_it = a.iter();
        let mut b_it = b.iter();
        let mut a_item = a_it.next();
        let mut b_item = b_it.next();

        while let (Some(a_value), Some(b_value)) = (a_item, b_item) {
            match a_value.cmp(&b_value) {
                Ordering::Less => a_item = a_it.next(),
                Ordering::Greater => b_item = b_it.next(),
                Ordering::Equal => {
                    a_item = a_it.next();
                    b_item = b_it.next();
                }
            }
        }

        for _ in a_it {}
        for _ in b_it {}
    });
}

fn union_vector(bencher: &mut Bencher) {
    let v0 = Vector::from(rand_array(FIRST_ARRAY_SIZE, MAX_ARRAY_VALUE));
    let v1 = Vector::from(rand_array(SECOND_ARRAY_SIZE, MAX_ARRAY_VALUE));

    bencher.iter(|| {
        let mut v0_it = v0.iter();
        let mut v1_it = v1.iter();
        let mut v0_item = v0_it.next();
        let mut v1_item = v1_it.next();

        while let (Some(v0_value), Some(v1_value)) = (v0_item, v1_item) {
            let mut rx = Bitset::default();

            match v0_value.base.cmp(&v1_value.base) {
                Ordering::Less => {
                    rx.bytes_to_bits(v0_value.data);
                    v0_item = v0_it.next();
                }
                Ordering::Greater => {
                    rx.bytes_to_bits(v1_value.data);
                    v1_item = v1_it.next();
                }
                Ordering::Equal => {
                    rx.bytes_to_bits(v0_value.data);
                    rx.bytes_to_bits(v1_value.data);
                    v0_item = v0_it.next();
                    v1_item = v1_it.next();
                }
            }
        }

        for value in v0_it {
            let mut rx = Bitset::default();
            rx.bytes_to_bits(value.data);
        }

        for value in v1_it {
            let mut rx = Bitset::default();
            rx.bytes_to_bits(value.data);
        }
    });
}

benchmark_group!(
    benches,
    intersect_array,
    intersect_vector,
    union_array,
    union_vector
);

benchmark_main!(benches);
