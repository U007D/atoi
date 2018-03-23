#![feature(test)]
extern crate test;

mod safe_atoi;
mod safe_atoi_v2;

use std::u64;
use test::Bencher;
use safe_atoi::safe_atoi;
use safe_atoi_v2::safe_atoi_v2;

fn main() {
    assert_eq!(safe_atoi_v2("0"), 0);
    assert_eq!(safe_atoi_v2("1"), 1);
    assert_eq!(safe_atoi_v2("2"), 2);
    assert_eq!(safe_atoi_v2("3"), 3);
    assert_eq!(safe_atoi_v2("4"), 4);
    assert_eq!(safe_atoi_v2("5"), 5);
    assert_eq!(safe_atoi_v2("6"), 6);
    assert_eq!(safe_atoi_v2("7"), 7);
    assert_eq!(safe_atoi_v2("18446744073709551608"), u64::MAX - 7);
    assert_eq!(safe_atoi_v2("18446744073709551609"), u64::MAX - 6);
    assert_eq!(safe_atoi_v2("18446744073709551610"), u64::MAX - 5);
    assert_eq!(safe_atoi_v2("18446744073709551611"), u64::MAX - 4);
    assert_eq!(safe_atoi_v2("18446744073709551612"), u64::MAX - 3);
    assert_eq!(safe_atoi_v2("18446744073709551613"), u64::MAX - 2);
    assert_eq!(safe_atoi_v2("18446744073709551614"), u64::MAX - 1);
    assert_eq!(safe_atoi_v2("18446744073709551615"), u64::MAX);
}

#[bench]
fn test_v1(bench: &mut Bencher) -> () {
    bench.iter(|| {
        assert_eq!(safe_atoi("0"), 0);
        assert_eq!(safe_atoi("1"), 1);
        assert_eq!(safe_atoi("2"), 2);
        assert_eq!(safe_atoi("3"), 3);
        assert_eq!(safe_atoi("4"), 4);
        assert_eq!(safe_atoi("5"), 5);
        assert_eq!(safe_atoi("6"), 6);
        assert_eq!(safe_atoi("7"), 7);
        assert_eq!(safe_atoi("18446744073709551608"), u64::MAX - 7);
        assert_eq!(safe_atoi("18446744073709551609"), u64::MAX - 6);
        assert_eq!(safe_atoi("18446744073709551610"), u64::MAX - 5);
        assert_eq!(safe_atoi("18446744073709551611"), u64::MAX - 4);
        assert_eq!(safe_atoi("18446744073709551612"), u64::MAX - 3);
        assert_eq!(safe_atoi("18446744073709551613"), u64::MAX - 2);
        assert_eq!(safe_atoi("18446744073709551614"), u64::MAX - 1);
        assert_eq!(safe_atoi("18446744073709551615"), u64::MAX);
    });
}

#[bench]
fn test_v2(bench: &mut Bencher) -> () {
    bench.iter(|| {
        assert_eq!(safe_atoi_v2("0"), 0);
        assert_eq!(safe_atoi_v2("1"), 1);
        assert_eq!(safe_atoi_v2("2"), 2);
        assert_eq!(safe_atoi_v2("3"), 3);
        assert_eq!(safe_atoi_v2("4"), 4);
        assert_eq!(safe_atoi_v2("5"), 5);
        assert_eq!(safe_atoi_v2("6"), 6);
        assert_eq!(safe_atoi_v2("7"), 7);
        assert_eq!(safe_atoi_v2("18446744073709551608"), u64::MAX - 7);
        assert_eq!(safe_atoi_v2("18446744073709551609"), u64::MAX - 6);
        assert_eq!(safe_atoi_v2("18446744073709551610"), u64::MAX - 5);
        assert_eq!(safe_atoi_v2("18446744073709551611"), u64::MAX - 4);
        assert_eq!(safe_atoi_v2("18446744073709551612"), u64::MAX - 3);
        assert_eq!(safe_atoi_v2("18446744073709551613"), u64::MAX - 2);
        assert_eq!(safe_atoi_v2("18446744073709551614"), u64::MAX - 1);
        assert_eq!(safe_atoi_v2("18446744073709551615"), u64::MAX);
    });
}
