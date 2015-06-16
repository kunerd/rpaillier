use num::bigint::{ BigInt, RandBigInt, Sign };
use num::traits::{ Zero, One, FromPrimitive };
use num::integer::Integer;
use rand::{ OsRng, StdRng };

mod key_pair;
pub use self::key_pair::KeyPair;

use public_key::PublicKey;
use private_key::PrivateKey;

use bigint_extensions::{ Two, Three, ModPow, ModInverse };

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

        let p_minus_one = p - BigInt::one();
        let q_minus_one = q - BigInt::one();

        let lambda = Integer::lcm(&p_minus_one, &q_minus_one);

        let mut g;
        let mut helper;

        loop {
        // while {
            g = BigInt::from_biguint(Sign::Plus, sec_rng.gen_biguint(self.bits));
            helper = calculate_l(&g.mod_pow(&lambda, &n_squared), &n);

            let a = helper.gcd(&n);
            if a == BigInt::one() {
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

fn calculate_l(u: &BigInt, n: &BigInt) -> BigInt{
    let r = u - BigInt::one();
    r / n
}


fn generate_possible_prime(sec_rng: &mut OsRng, bits: usize, certainty: u32) -> BigInt {
    let mut pp;

    'outer:
    loop {
        pp = BigInt::from_biguint(Sign::Plus, sec_rng.gen_biguint(bits));
        if pp.is_even() {
            continue;
        }

        let primes = [ 2, 3, 5, 7, 11, 13, 17, 19, 23 ];
        for prime in primes.iter() {
            let big_prime = BigInt::from_u64(*prime).unwrap();
            if &pp % big_prime == BigInt::zero() {
                continue 'outer;
            }
        }

        if miller_rabin(&pp, certainty) {
            break;
        }
    }
    return pp;
}

fn miller_rabin(n: &BigInt, k: u32) -> bool{
    if n <= &BigInt::three() {
        return true;
    }

    let n_minus_one = n - BigInt::one();

    let mut s = 0;
    let mut r = n_minus_one.clone();

    while &r % &BigInt::two() == BigInt::zero() {
       s += 1;
       r = r / BigInt::two();
    }

    let mut rng = match StdRng::new() {
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS RNG: {}", e)
    };

   let mut a = BigInt::two();
   for _ in 0..k {
       let mut x = a.mod_pow(&r, &n);

       if x == BigInt::one() || x == n_minus_one {
            continue;
        }

       for _ in 1..(s - 1) {
           x = &x * &x % n;
           if x == BigInt::one(){
               return false;
           }
        }
        if x != n_minus_one{
            return false;
        }

        a = rng.gen_bigint_range(&BigInt::two(), &n_minus_one);
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
