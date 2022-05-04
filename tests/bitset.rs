use vector::{Bitset, BITS_SIZE};

#[test]
fn convert_bytes_to_bits() {
    let mut bitset = Bitset::default();
    bitset.bytes_to_bits(&[0, 65, 66, 129, 130, 255]);
    assert_eq!(bitset.as_bits(), [1, 6, 6, 9223372036854775808]);
}

#[test]
fn convert_bits_to_bytes_dump() {
    let bitset = Bitset::new([1, 6, 6, 9223372036854775808]);
    let bytes = bitset.bits_to_bytes_dump(vec![0; BITS_SIZE]);
    assert_eq!(bytes[0..6], [0, 65, 66, 129, 130, 255]);
}

#[test]
fn convert_bits_to_bytes() {
    let bitset = Bitset::new([1, 6, 6, 9223372036854775808]);
    let mut bytes = vec![0; BITS_SIZE];
    bitset.bits_to_bytes(&mut bytes);
    assert_eq!(bytes[0..6], [0, 65, 66, 129, 130, 255]);
}

#[test]
fn is_zero() {
    assert!(Bitset::new([0, 0, 0, 0]).is_zero());
    assert!(!Bitset::new([0, 0, 0, 1]).is_zero());
    assert!(!Bitset::new([0, 0, 1, 0]).is_zero());
    assert!(!Bitset::new([0, 1, 0, 0]).is_zero());
    assert!(!Bitset::new([1, 0, 0, 0]).is_zero());
    assert!(!Bitset::new([1, 1, 1, 1]).is_zero());
}

#[test]
fn bit_and() {
    let x = Bitset::new([1, 2, 3, 4]) & Bitset::new([5, 6, 7, 8]);
    assert_eq!(x, Bitset::new([1, 2, 3, 0]));
}

#[test]
fn bit_and_assign() {
    let mut x = Bitset::new([1, 2, 3, 4]);
    x &= Bitset::new([5, 6, 7, 8]);
    assert_eq!(x, Bitset::new([1, 2, 3, 0]));
}

#[test]
fn bit_or() {
    let x = Bitset::new([1, 2, 3, 4]) | Bitset::new([5, 6, 7, 8]);
    assert_eq!(x, Bitset::new([5, 6, 7, 12]));
}

#[test]
fn bit_or_assign() {
    let mut x = Bitset::new([1, 2, 3, 4]);
    x |= Bitset::new([5, 6, 7, 8]);
    assert_eq!(x, Bitset::new([5, 6, 7, 12]));
}

#[test]
fn bit_xor() {
    let x = Bitset::new([1, 2, 3, 4]) ^ Bitset::new([5, 6, 7, 8]);
    assert_eq!(x, Bitset::new([4, 4, 4, 12]));
}

#[test]
fn bit_xor_assign() {
    let mut x = Bitset::new([1, 2, 3, 4]);
    x ^= Bitset::new([5, 6, 7, 8]);
    assert_eq!(x, Bitset::new([4, 4, 4, 12]));
}
