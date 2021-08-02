use factorial::Factorial;

pub struct Wigner3nj1st {
    pub js: Vec<u128>,
    pub ls: Vec<u128>,
    pub ks: Vec<u128>,
}

impl Wigner3nj1st {
    pub fn value(self) -> f64 {
        let Wigner3nj1st { mut js, ls, mut ks } = self;

        let kmin = 0;
        let kmax = js[0] + ks[0];
        let n = ls.len();
        let overall_phase = phase(
            (js.iter().sum::<u128>() + ls.iter().sum::<u128>() + ks.iter().sum::<u128>()) / 2,
        );
        let mut value = 0.0;
        js.push(js[0]);
        ks.push(ks[0]);
        for i in 0..(kmax - kmin) / 2 + 1 {
            let x = kmin + 2 * i;
            let mut wprd = 1.0;
            for i in 0..n {
                let w = Wigner6j {
                    j1: js[i],
                    j2: ks[i],
                    j3: x,
                    j4: ks[i + 1],
                    j5: js[i + 1],
                    j6: ls[i],
                };
                wprd *= w.value();
            }
            value += (x + 1) as f64 * phase((n as u128 - 1) * x / 2) * wprd;
        }
        value * overall_phase
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
