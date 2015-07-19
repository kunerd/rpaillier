use ramp::{ Int, RandomInt };
use bigint_extensions::ModPow;
use rand::{ StdRng };

mod builder;
pub use self::builder::KeyPairBuilder;

pub struct KeyPair {
    pub public_key: PublicKey,
    private_key: PrivateKey
}

impl KeyPair {
    pub fn decrypt(&self, c: &Int) -> Int {

        let n = &self.public_key.n;
        let n_square = &self.public_key.n_squared;

        let lambda = &self.private_key.lambda;
        let u = &self.private_key.denominator;

        ((c.mod_pow(&lambda, &n_square) - Int::one()) / n * u) % n
    }
}

pub struct PublicKey {
    pub bits: usize,
    pub n: Int,
    pub n_squared: Int,
    pub g: Int
}

impl PublicKey {
    pub fn encrypt(&self, m: &Int) -> Int{
        let mut rng = match StdRng::new() {
            Ok(g) => g,
            Err(e) => panic!("Failed to obtain StdRng RNG: {}", e)
        };

        let r = rng.gen_uint_range(&self.n, &self.n_squared);

        let mut result = self.g.mod_pow(m, &self.n_squared);
        let x = r.mod_pow(&self.n, &self.n_squared);

        result = result * x;
        result = result % &self.n_squared;

        result
    }
}

struct PrivateKey {
    lambda: Int,
    denominator: Int
}
