use ramp::{ Int, RandomInt};
use rand::{ OsRng, StdRng };

use super::{ KeyPair, PublicKey, PrivateKey };

use bigint_extensions::{ ModPow, ModInverse };

pub struct KeyPairBuilder {
    bits: usize,
    certainty: u32
}

impl KeyPairBuilder {
    pub fn new() -> KeyPairBuilder {
        KeyPairBuilder { bits: 512, certainty: 4 }
    }

    pub fn bits(&mut self, bits: usize) -> &mut KeyPairBuilder {
        self.bits = bits;
        self
    }

    pub fn certainty(&mut self, certainty: u32) -> &mut KeyPairBuilder {
        self.certainty = certainty;
        self
    }

    pub fn finalize(&self) -> KeyPair {

        let mut sec_rng = match OsRng::new() {
            Ok(g) => g,
            Err(e) => panic!("Failed to obtain OS RNG: {}", e)
        };

        let p = &generate_possible_prime(&mut sec_rng, self.bits, self.certainty);
        let q = &generate_possible_prime(&mut sec_rng, self.bits, self.certainty);

        let n = p * q;
        let n_squared = &n * &n;

        let p_minus_one = p - Int::one();
        let q_minus_one = q - Int::one();

        let lambda = p_minus_one.lcm(&q_minus_one);

        let mut g;
        let mut helper;

        loop {
            g = sec_rng.gen_uint(self.bits);
            helper = calculate_l(&g.mod_pow(&lambda, &n_squared), &n);

            let a = helper.gcd(&n);
            if a == Int::one() {
                break;
            }
        }

        let public_key =
            PublicKey {
                bits: self.bits,
                n: n.clone(),
                n_squared: n_squared,
                g: g.clone()
            };

        let private_key = PrivateKey {
                lambda: lambda,
                denominator: helper.mod_inverse(&n).unwrap()
            };

        KeyPair { public_key: public_key, private_key: private_key }
    }

}

fn calculate_l(u: &Int, n: &Int) -> Int{
    let r = u - Int::one();
    r / n
}


fn generate_possible_prime(sec_rng: &mut OsRng, bits: usize, certainty: u32) -> Int {
    let mut pp;

    'outer:
    loop {
        pp = sec_rng.gen_uint(bits);
        if (&pp % &Int::from(2)) == Int::zero() {
            continue;
        }

        let primes = [ 2, 3, 5, 7, 11, 13, 17, 19, 23 ];
        for prime in primes.iter() {
            let big_prime = Int::from(*prime);
            if &pp % big_prime == Int::zero() {
                continue 'outer;
            }
        }

        if miller_rabin(&pp, certainty) {
            break;
        }
    }
    return pp;
}

fn miller_rabin(n: &Int, k: u32) -> bool{
    if n <= &Int::from(3) {
        return true;
    }

    let n_minus_one = n - Int::one();

    let mut s = 0;
    let mut r = n_minus_one.clone();

    let two = Int::from(2);
    while &r % &two == Int::zero() {
       s += 1;
       r = r / &two;
    }

    let mut rng = match StdRng::new() {
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS RNG: {}", e)
    };

   let mut a = Int::from(2);
   for _ in 0..k {
       let mut x = a.mod_pow(&r, &n);

       if x == Int::one() || x == n_minus_one {
            continue;
        }

       for _ in 1..(s - 1) {
           x = &x * &x % n;
           if x == Int::one(){
               return false;
           }
        }
        if x != n_minus_one{
            return false;
        }

        a = rng.gen_uint_range(&Int::from(2), &n_minus_one);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::generate_possible_prime;
    use rand::OsRng;

    use test::Bencher;

    #[bench]
    fn bench_generate_possible_prime(b: &mut Bencher) {
        let mut rng = match OsRng::new() {
            Ok(g) => g,
            Err(e) => panic!("Failed to obtain OS RNG: {}", e)
        };

        b.iter(|| {
            generate_possible_prime(&mut rng, 64, 10);
        });
    }

}
