use factorial::Factorial;

pub struct Wigner9j {
    pub j1: u128,
    pub j2: u128,
    pub j3: u128,
    pub l1: u128,
    pub l2: u128,
    pub l3: u128,
    pub k1: u128,
    pub k2: u128,
    pub k3: u128,
}

impl Wigner9j {
    pub fn value(self) -> f64 {
        let Wigner9j {
            j1,
            j2,
            j3,
            l1,
            l2,
            l3,
            k1,
            k2,
            k3,
        } = self;
        let min1: u128;
        let min2: u128;
        let min3: u128;
        if j1 > k1 {
            min1 = j1 - k1;
        } else {
            min1 = k1 - j1;
        }

        if j2 > k2 {
            min2 = j2 - k2;
        } else {
            min2 = k2 - j2;
        }

        if j3 > k3 {
            min3 = j3 - k3;
        } else {
            min3 = k3 - j3;
        }

        let kmin = [min1, min2, min3].iter().fold(0, |m, v| *v.max(&m));
        let kmax = [j1 + k1, j2 + k2, j3 + k3]
            .iter()
            .fold(u128::MAX, |m, v| *v.min(&m));
        let mut sum = 0.0;
        for i in 0..(kmax - kmin) / 2 + 1 {
            let x = kmin + 2 * i;
            let w1 = Wigner6j {
                j1: j1,
                j2: k1,
                j3: x,
                j4: k2,
                j5: j2,
                j6: l1,
            };
            let w2 = Wigner6j {
                j1: j2,
                j2: k2,
                j3: x,
                j4: k3,
                j5: j3,
                j6: l2,
            };
            let w3 = Wigner6j {
                j1: j3,
                j2: k3,
                j3: x,
                j4: k1,
                j5: j1,
                j6: l3,
            };
            sum += (x + 1) as f64
                * phase((j1 + j2 + j3 + l1 + l2 + l3 + k1 + k2 + k3) / 2 + 2 * x / 2)
                * w1.value()
                * w2.value()
                * w3.value();
        }
        sum
    }
}

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
        if violate_triad_conditions(j1, j2, j3) {
            return 0.0;
        }

        if violate_triad_conditions(j1, j5, j6) {
            return 0.0;
        }

        if violate_triad_conditions(j4, j2, j6) {
            return 0.0;
        }

        if violate_triad_conditions(j4, j5, j3) {
            return 0.0;
        }

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

fn violate_triad_conditions(j1: u128, j2: u128, j3: u128) -> bool {
    if j2 > j3 {
        j1 < j2 - j3 || j2 + j3 < j1 || (j1 + j2 + j3) % 2 == 1
    } else {
        j1 < j3 - j2 || j2 + j3 < j1 || (j1 + j2 + j3) % 2 == 1
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
