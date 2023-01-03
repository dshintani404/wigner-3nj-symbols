# wigner-3nj-symbols

wigner_3nj_symbols is a crate to calculate 3n-j symbols for arbitrary positive integer n.

This crate does not support arbitrary-precision arithmetic.

Definitions of 3n-j symbols are based on equations (17.1) and (17.2) of the following textbook.

A. P. Yutsis, I. B. Levinson, and V. V. Vanagas, Mathematical Apparatus of the Theory of Angular Momentum (Israel Program for Scientific Translations, Jerusalem, 1962).

## 3n-j Symobols of the first kind

```
use wigner_3nj_symbols::Wigner3nj1st;
let w = Wigner3nj1st {
    js: vec!(1,1,1),
    ls: vec!(2,2,2),
    ks: vec!(1,1,1)
};
assert_eq!(w.value(), 0.1388888888888889)
```

## 3n-j Symobols of the second kind

```
use wigner_3nj_symbols::Wigner3nj2nd;
let w = Wigner3nj2nd {
    js: vec!(1,1,1),
    ls: vec!(2,2,2),
    ks: vec!(1,1,1)
};
assert_eq!(w.value(), 0.1111111111111111)
```
