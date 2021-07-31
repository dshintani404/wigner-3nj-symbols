extern crate wigner_3nj_symbols;

use wigner_3nj_symbols::{Wigner3nj, Wigner6j};

#[test]
fn test_wigner_6j_1() {
    let w = Wigner6j {
        j1: 1,
        j2: 1,
        j3: 0,
        j4: 1,
        j5: 1,
        j6: 2,
    };
    assert_eq!(w.value(), 0.5)
}

#[test]
fn test_wigner_6j_2() {
    let w = Wigner6j {
        j1: 1,
        j2: 1,
        j3: 2,
        j4: 1,
        j5: 1,
        j6: 2,
    };
    assert_eq!(w.value(), 0.16666666666666666)
}

#[test]
fn test_wigner_6j_3() {
    let w = Wigner6j {
        j1: 2,
        j2: 4,
        j3: 6,
        j4: 8,
        j5: 10,
        j6: 12,
    };
    assert_eq!(w.value(), 0.017629529511598168)
}

#[test]
fn test_wigner_6j_4() {
    let w = Wigner6j {
        j1: 0,
        j2: 0,
        j3: 0,
        j4: 1,
        j5: 1,
        j6: 0,
    };
    assert_eq!(w.value(), 0.0)
}

#[test]
fn test_wigner_3nj_0() {
    let w = Wigner3nj {
        js: vec![1, 1, 0],
        ls: vec![1, 1, 0],
        ks: vec![1, 1, 0],
    };
    assert_eq!(w.value(), 0.0)
}

#[test]
fn test_wigner_3nj_1() {
    let w = Wigner3nj {
        js: vec![1, 1, 1],
        ls: vec![2, 2, 2],
        ks: vec![1, 1, 1],
    };
    assert_eq!(w.value(), 0.1388888888888889)
}

#[test]
fn test_wigner_3nj_2() {
    let w = Wigner3nj {
        js: vec![3, 1, 1],
        ls: vec![2, 2, 2],
        ks: vec![1, 1, 1],
    };
    assert_eq!(w.value(), -0.05555555555555555)
}
