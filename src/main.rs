#![feature(test)]
extern crate test;

mod atoi_v1;
mod atoi;

use std::u64;
#[allow(unused)]
use test::Bencher;
#[allow(unused)]
use atoi_v1::atoi_v1;
use atoi::*;

fn main() {
    // atoi_v4
    assert_eq!(atoi_v4("0"), 0);
    assert_eq!(atoi_v4("1"), 1);
    assert_eq!(atoi_v4("2"), 2);
    assert_eq!(atoi_v4("3"), 3);
    assert_eq!(atoi_v4("4"), 4);
    assert_eq!(atoi_v4("5"), 5);
    assert_eq!(atoi_v4("6"), 6);
    assert_eq!(atoi_v4("7"), 7);
    assert_eq!(atoi_v4("18446744073709551608"), u64::MAX - 7);
    assert_eq!(atoi_v4("18446744073709551609"), u64::MAX - 6);
    assert_eq!(atoi_v4("18446744073709551610"), u64::MAX - 5);
    assert_eq!(atoi_v4("18446744073709551611"), u64::MAX - 4);
    assert_eq!(atoi_v4("18446744073709551612"), u64::MAX - 3);
    assert_eq!(atoi_v4("18446744073709551613"), u64::MAX - 2);
    assert_eq!(atoi_v4("18446744073709551614"), u64::MAX - 1);
    assert_eq!(atoi_v4("18446744073709551615"), u64::MAX);
}

#[bench]
fn test_v1(bench: &mut Bencher) -> () {
    bench.iter(|| {
        assert_eq!(atoi_v1("0"), 0);
        assert_eq!(atoi_v1("1"), 1);
        assert_eq!(atoi_v1("2"), 2);
        assert_eq!(atoi_v1("3"), 3);
        assert_eq!(atoi_v1("4"), 4);
        assert_eq!(atoi_v1("5"), 5);
        assert_eq!(atoi_v1("6"), 6);
        assert_eq!(atoi_v1("7"), 7);
        assert_eq!(atoi_v1("18446744073709551608"), u64::MAX - 7);
        assert_eq!(atoi_v1("18446744073709551609"), u64::MAX - 6);
        assert_eq!(atoi_v1("18446744073709551610"), u64::MAX - 5);
        assert_eq!(atoi_v1("18446744073709551611"), u64::MAX - 4);
        assert_eq!(atoi_v1("18446744073709551612"), u64::MAX - 3);
        assert_eq!(atoi_v1("18446744073709551613"), u64::MAX - 2);
        assert_eq!(atoi_v1("18446744073709551614"), u64::MAX - 1);
        assert_eq!(atoi_v1("18446744073709551615"), u64::MAX);
    });
}

#[bench]
fn test_v2(bench: &mut Bencher) -> () {
    bench.iter(|| {
        assert_eq!(atoi_v2("0"), 0);
        assert_eq!(atoi_v2("1"), 1);
        assert_eq!(atoi_v2("2"), 2);
        assert_eq!(atoi_v2("3"), 3);
        assert_eq!(atoi_v2("4"), 4);
        assert_eq!(atoi_v2("5"), 5);
        assert_eq!(atoi_v2("6"), 6);
        assert_eq!(atoi_v2("7"), 7);
        assert_eq!(atoi_v2("18446744073709551608"), u64::MAX - 7);
        assert_eq!(atoi_v2("18446744073709551609"), u64::MAX - 6);
        assert_eq!(atoi_v2("18446744073709551610"), u64::MAX - 5);
        assert_eq!(atoi_v2("18446744073709551611"), u64::MAX - 4);
        assert_eq!(atoi_v2("18446744073709551612"), u64::MAX - 3);
        assert_eq!(atoi_v2("18446744073709551613"), u64::MAX - 2);
        assert_eq!(atoi_v2("18446744073709551614"), u64::MAX - 1);
        assert_eq!(atoi_v2("18446744073709551615"), u64::MAX);
    });
}

#[bench]
fn test_v3(bench: &mut Bencher) -> () {
    bench.iter(|| {
        assert_eq!(atoi_v3("0"), 0);
        assert_eq!(atoi_v3("1"), 1);
        assert_eq!(atoi_v3("2"), 2);
        assert_eq!(atoi_v3("3"), 3);
        assert_eq!(atoi_v3("4"), 4);
        assert_eq!(atoi_v3("5"), 5);
        assert_eq!(atoi_v3("6"), 6);
        assert_eq!(atoi_v3("7"), 7);
        assert_eq!(atoi_v3("18446744073709551608"), u64::MAX - 7);
        assert_eq!(atoi_v3("18446744073709551609"), u64::MAX - 6);
        assert_eq!(atoi_v3("18446744073709551610"), u64::MAX - 5);
        assert_eq!(atoi_v3("18446744073709551611"), u64::MAX - 4);
        assert_eq!(atoi_v3("18446744073709551612"), u64::MAX - 3);
        assert_eq!(atoi_v3("18446744073709551613"), u64::MAX - 2);
        assert_eq!(atoi_v3("18446744073709551614"), u64::MAX - 1);
        assert_eq!(atoi_v3("18446744073709551615"), u64::MAX);
    });
}

#[bench]
fn test_v4(bench: &mut Bencher) -> () {
    bench.iter(|| {
        assert_eq!(atoi_v4("0"), 0);
        assert_eq!(atoi_v4("1"), 1);
        assert_eq!(atoi_v4("2"), 2);
        assert_eq!(atoi_v4("3"), 3);
        assert_eq!(atoi_v4("4"), 4);
        assert_eq!(atoi_v4("5"), 5);
        assert_eq!(atoi_v4("6"), 6);
        assert_eq!(atoi_v4("7"), 7);
        assert_eq!(atoi_v4("18446744073709551608"), u64::MAX - 7);
        assert_eq!(atoi_v4("18446744073709551609"), u64::MAX - 6);
        assert_eq!(atoi_v4("18446744073709551610"), u64::MAX - 5);
        assert_eq!(atoi_v4("18446744073709551611"), u64::MAX - 4);
        assert_eq!(atoi_v4("18446744073709551612"), u64::MAX - 3);
        assert_eq!(atoi_v4("18446744073709551613"), u64::MAX - 2);
        assert_eq!(atoi_v4("18446744073709551614"), u64::MAX - 1);
        assert_eq!(atoi_v4("18446744073709551615"), u64::MAX);
    });
}

#[bench]
fn test_v5(bench: &mut Bencher) -> () {
    bench.iter(|| {
        assert_eq!(atoi_v4("0"), 0);
        assert_eq!(atoi_v4("1"), 1);
        assert_eq!(atoi_v4("2"), 2);
        assert_eq!(atoi_v4("3"), 3);
        assert_eq!(atoi_v4("4"), 4);
        assert_eq!(atoi_v4("5"), 5);
        assert_eq!(atoi_v4("6"), 6);
        assert_eq!(atoi_v4("7"), 7);
        assert_eq!(atoi_v4("18446744073709551608"), u64::MAX - 7);
        assert_eq!(atoi_v4("18446744073709551609"), u64::MAX - 6);
        assert_eq!(atoi_v4("18446744073709551610"), u64::MAX - 5);
        assert_eq!(atoi_v4("18446744073709551611"), u64::MAX - 4);
        assert_eq!(atoi_v4("18446744073709551612"), u64::MAX - 3);
        assert_eq!(atoi_v4("18446744073709551613"), u64::MAX - 2);
        assert_eq!(atoi_v4("18446744073709551614"), u64::MAX - 1);
        assert_eq!(atoi_v4("18446744073709551615"), u64::MAX);
    });
}
