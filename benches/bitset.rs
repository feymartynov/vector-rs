#[macro_use]
extern crate bencher;

use bencher::Bencher;

use vector::bitset::Bitset;

fn convert_bytes_to_bits(b: &mut Bencher) {
    let mut bitset = Bitset::default();
    b.iter(|| bitset.bytes_to_bits(&[1, 4, 128, 200, 240]));
}

fn bits_to_bytes_dump<const N: usize>(b: &mut Bencher) {
    let bitsets = (0..N)
        .map(|i| Bitset::new([i as u64; 4]))
        .collect::<Vec<_>>();

    b.iter(|| {
        for bitset in &bitsets {
            let _ = bitset.bits_to_bytes_dump(vec![0; 256]);
        }
    })
}

fn bits_to_bytes<const N: usize>(b: &mut Bencher) {
    let bitsets = (0..N)
        .map(|i| Bitset::new([i as u64; 4]))
        .collect::<Vec<_>>();

    b.iter(|| {
        for bitset in &bitsets {
            let _ = bitset.bits_to_bytes(vec![0; 256]);
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
