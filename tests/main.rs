extern crate wigner_3nj_symbols;

use wigner_3nj_symbols::Wigner6j;

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
