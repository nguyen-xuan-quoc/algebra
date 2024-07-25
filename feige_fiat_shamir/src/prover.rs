use num_bigint::BigUint;
use rand::Rng;

#[derive(Debug)]
pub struct Prover {
    pub n: BigUint,
    s: Vec<BigUint>,
    pub v: Vec<BigUint>,
}

impl Prover {
    pub fn new(p: BigUint, q: BigUint, s: Vec<BigUint>) -> Self {
        let n = &p * &q;
        let mut v: Vec<BigUint> = Vec::new();
        for si in s.iter() {
            let vi = si.modpow(&BigUint::from(2_u32), &n);
            v.push(vi);
        }
        Prover { n, s, v }
    }

    pub fn gen_sign(&self) -> i32 {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..2) {
            0 => -1,
            1 => 1,
            _ => panic!("Invalid sign"),
        }
    }

    pub fn commit(&self) -> (BigUint, BigUint) {
        let mut rng = rand::thread_rng();
        let r: BigUint = rng.sample(num_bigint::RandomBits::new(256));
        let s: i32 = self.gen_sign();

        match s {
            1 => {
                let x = r.modpow(&BigUint::from(2_u32), &self.n);
                (r, x)
            }
            -1 => {
                let k = r.modpow(&BigUint::from(2_u32), &self.n);
                let x = &self.n - &k;
                (r, x)
            }
            _ => panic!("Invalid sign"),
        }
    }

    pub fn respond(&self, r: &BigUint, a: &[u32]) -> BigUint {
        let mut rs = BigUint::from(1_u32);
        rs *= r;
        for (i, ai) in a.iter().enumerate() {
            rs *= self.s[i].modpow(&BigUint::from(*ai), &self.n);
            rs %= &self.n;
        }
        rs
    }
}
