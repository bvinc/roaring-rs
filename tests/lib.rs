extern crate roaring;
use roaring::RoaringBitmap;

use std::iter::FromIterator;

#[test]
fn smoke() {
    let mut bitmap = RoaringBitmap::new();
    assert_eq!(bitmap.len(), 0);
    assert_eq!(bitmap.is_empty(), true);
    bitmap.remove(0);
    assert_eq!(bitmap.len(), 0);
    assert_eq!(bitmap.is_empty(), true);
    bitmap.insert(1);
    assert_eq!(bitmap.contains(1), true);
    assert_eq!(bitmap.len(), 1);
    assert_eq!(bitmap.is_empty(), false);
    bitmap.insert(u32::max_value() - 2);
    assert_eq!(bitmap.contains(u32::max_value() - 2), true);
    assert_eq!(bitmap.len(), 2);
    bitmap.insert(u32::max_value());
    assert_eq!(bitmap.contains(u32::max_value()), true);
    assert_eq!(bitmap.len(), 3);
    bitmap.insert(2);
    assert_eq!(bitmap.contains(2), true);
    assert_eq!(bitmap.len(), 4);
    bitmap.remove(2);
    assert_eq!(bitmap.contains(2), false);
    assert_eq!(bitmap.len(), 3);
    assert_eq!(bitmap.contains(0), false);
    assert_eq!(bitmap.contains(1), true);
    assert_eq!(bitmap.contains(100), false);
    assert_eq!(bitmap.contains(u32::max_value() - 2), true);
    assert_eq!(bitmap.contains(u32::max_value() - 1), false);
    assert_eq!(bitmap.contains(u32::max_value()), true);
}

#[test]
fn to_bitmap() {
    let bitmap = RoaringBitmap::from_iter(0..5000);
    assert_eq!(bitmap.len(), 5000);
    for i in 1..5000{
        assert_eq!(bitmap.contains(i), true);
    }
    assert_eq!(bitmap.contains(5001), false);
}

#[test]
fn to_array() {
    let mut bitmap = RoaringBitmap::from_iter(0..5000);
    for i in 3000..5000 {
        bitmap.remove(i);
    }
    assert_eq!(bitmap.len(), 3000);
    for i in 0..3000 {
        assert_eq!(bitmap.contains(i), true);
    }
    for i in 3000..5000 {
        assert_eq!(bitmap.contains(i), false);
    }
}
