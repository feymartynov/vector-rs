use vector::Vector;

#[test]
fn add() {
    let mut vector = Vector::new();
    vector.add(1);
    vector.add(3);
    vector.add(2000);
    assert_eq!(vector.as_bytes(), &[1, 3, 208]);
    assert_eq!(vector.len(), 11);
    assert_eq!(vector.capacity(), 24);
}

#[test]
fn add_last() {
    let mut vector = Vector::new();
    vector.add(1);
    vector.add(1);
    assert_eq!(vector.as_bytes(), &[1]);
    assert_eq!(vector.len(), 5);
    assert_eq!(vector.capacity(), 24);
}

#[test]
fn from_values() {
    let vector = Vector::from([1, 3, 2]);
    assert!(!vector.is_empty());
    assert_eq!(vector.as_bytes(), &[1, 3, 2]);
    assert_eq!(vector.len(), 7);
    assert_eq!(vector.capacity(), 24);
}

#[test]
fn from_range() {
    let vector = Vector::from(1..5);
    assert!(!vector.is_empty());
    assert_eq!(vector.as_bytes(), &[1, 2, 3, 4]);
    assert_eq!(vector.len(), 8);
    assert_eq!(vector.capacity(), 24);
}

#[test]
fn reset() {
    let mut vector = Vector::from([1, 3, 2]);
    vector.reset();
    assert!(vector.is_empty());
}

#[test]
fn iter() {
    let vector = Vector::from([1, 2, 1000, 1001, 2000]);
    let mut it = vector.iter();
    assert_eq!(it.size_hint(), (2, Some(2)));

    let item = it.next().expect("Expected item");
    assert_eq!(item.base, 3);
    assert_eq!(item.size, 1);
    assert_eq!(item.data, [2]);

    let item = it.next().expect("Expected item");
    assert_eq!(item.base, 7);
    assert_eq!(item.size, 0);
    assert!(item.data.is_empty());

    assert!(it.next().is_none());
    it.reset();
    assert_eq!(it.size_hint(), (2, Some(2)));

    let item = it.next().expect("Expected item");
    assert_eq!(item.base, 3);
    assert_eq!(item.size, 1);
    assert_eq!(item.data, [2]);

    let item = it.next().expect("Expected item");
    assert_eq!(item.base, 7);
    assert_eq!(item.size, 0);
    assert!(item.data.is_empty());

    assert!(it.next().is_none());
}
