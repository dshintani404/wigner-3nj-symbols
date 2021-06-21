use factorial::Factorial;

pub struct Wigner6j {
    pub j1: u128,
    pub j2: u128,
    pub j3: u128,
    pub j4: u128,
    pub j5: u128,
    pub j6: u128,
}

impl Wigner6j {
    pub fn value(self) -> f64 {
        let Wigner6j {
            j1,
            j2,
            j3,
            j4,
            j5,
            j6,
        } = self;
        let prod_delta =
            delta(j1, j2, j3) * delta(j4, j5, j3) * delta(j1, j5, j6) * delta(j4, j2, j6);
        let mut sum = 0.0;
        let mut numerator;
        let mut denominator;
        let kmin = [j1 + j2 + j3, j4 + j5 + j3, j1 + j5 + j6, j4 + j2 + j6]
            .iter()
            .fold(0, |m, v| *v.max(&m));
        let kmax = [j1 + j2 + j4 + j5, j1 + j3 + j4 + j6, j2 + j3 + j5 + j6]
            .iter()
            .fold(u128::MAX, |m, v| *v.min(&m));
        let mut k;
        for i in 0..(kmax - kmin) / 2 + 1 {
            k = kmin + i * 2;
            numerator = phase(k / 2) * (k / 2 + 1).factorial() as f64;
            denominator = ((k - j1 - j2 - j3) / 2).factorial()
                * ((k - j4 - j5 - j3) / 2).factorial()
                * ((k - j1 - j5 - j6) / 2).factorial()
                * ((k - j4 - j2 - j6) / 2).factorial()
                * ((j1 + j2 + j4 + j5 - k) / 2).factorial()
                * ((j1 + j3 + j4 + j6 - k) / 2).factorial()
                * ((j2 + j3 + j5 + j6 - k) / 2).factorial();
            sum += numerator / denominator as f64
        }

        prod_delta.sqrt() * sum
    }
}

fn phase(j: u128) -> f64 {
    if j % 2 == 0 {
        1.0
    } else {
        -1.0
    }
}

fn delta(j1: u128, j2: u128, j3: u128) -> f64 {
    let numerator = ((j1 + j2 - j3) / 2).factorial()
        * ((j3 + j1 - j2) / 2).factorial()
        * ((j2 + j3 - j1) / 2).factorial();
    let denominator = ((j1 + j2 + j3) / 2 + 1).factorial() as f64;
    (numerator as f64) / denominator
}
