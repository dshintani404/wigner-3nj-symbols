extern crate wigner_3nj_symbols;

use rstest::rstest;
use wigner_3nj_symbols::{Wigner3nj1st, Wigner3nj2nd, Wigner6j};

#[rstest]
#[case(0, 0, 0, 1, 1, 0, 0.0)]
#[case(1, 1, 0, 1, 1, 2, 0.5)]
#[case(1, 1, 2, 1, 1, 2, 0.16666666666666666)]
#[case(2, 4, 6, 8, 10, 12, 0.017629529511598168)]
fn test_wigner_6j(
    #[case] j1: u128,
    #[case] j2: u128,
    #[case] j3: u128,
    #[case] j4: u128,
    #[case] j5: u128,
    #[case] j6: u128,
    #[case] expected: f64,
) {
    let w = Wigner6j {
        j1: j1,
        j2: j2,
        j3: j3,
        j4: j4,
        j5: j5,
        j6: j6,
    };
    assert_eq!(w.value(), expected)
}

#[rstest]
#[case(vec![1,1,0], vec![1,1,0], vec![1,1,0], 0.0)]
#[case(vec![1,1,1], vec![2,2,2], vec![1,1,1], 0.1388888888888889)]
#[case(vec![3,1,1], vec![2,2,2], vec![1,1,1], -0.05555555555555555)]
fn test_wigner_3nj_1st(
    #[case] js: Vec<u128>,
    #[case] ls: Vec<u128>,
    #[case] ks: Vec<u128>,
    #[case] expected: f64,
) {
    let w = Wigner3nj1st {
        js: js,
        ls: ls,
        ks: ks,
    };
    assert_eq!(w.value(), expected)
}

#[rstest]
#[case(vec![1,1,0], vec![1,1,0], vec![1,1,0], 0.0)]
#[case(vec![1,1,1], vec![2,2,2], vec![1,1,1], 0.1111111111111111)]
#[case(vec![3,1,1], vec![2,2,2], vec![1,1,1], 0.05555555555555555)]
fn test_wigner_3nj_2nd(
    #[case] js: Vec<u128>,
    #[case] ls: Vec<u128>,
    #[case] ks: Vec<u128>,
    #[case] expected: f64,
) {
    let w = Wigner3nj2nd {
        js: js,
        ls: ls,
        ks: ks,
    };
    assert_eq!(w.value(), expected)
}
