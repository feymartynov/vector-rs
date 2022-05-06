#[macro_use]
extern crate bencher;

use bencher::Bencher;
use vector::{Bitset, BITS_SIZE};

fn convert_bytes_to_bits(bencher: &mut Bencher) {
    let mut bitset = Bitset::default();
    bencher.iter(|| bitset.load_bytes(&[1, 4, 128, 200, 240]));
}

fn bits_to_bytes_dump<const N: usize>(bencher: &mut Bencher) {
    let bitsets = (0..N)
        .map(|i| Bitset::new([i as u64; 4]))
        .collect::<Vec<_>>();
        
    let mut bytes = vec![0; BITS_SIZE];

    bencher.iter(|| {
        for bitset in &bitsets {
            bitset.to_bytes_dump(&mut bytes);
        }
    })
}

fn bits_to_bytes<const N: usize>(bencher: &mut Bencher) {
    let bitsets = (0..N)
        .map(|i| Bitset::new([i as u64; 4]))
        .collect::<Vec<_>>();

    let mut bytes = [0; BITS_SIZE];

    bencher.iter(|| {
        for bitset in &bitsets {
            bitset.to_bytes(&mut bytes);
        }
    })
}

benchmark_group!(
    benches,
    convert_bytes_to_bits,
    bits_to_bytes_dump::<100_000>,
    bits_to_bytes::<100_000>,
    bits_to_bytes_dump::<500_000>,
    bits_to_bytes::<500_000>,
);

benchmark_main!(benches);
