use num::bigint::{ BigInt, RandBigInt };
use rand::StdRng;
use bigint_extensions::ModPow;


pub struct PublicKey {
    pub bits: usize,
    pub n: BigInt,
    pub n_squared: BigInt,
    pub g: BigInt
}

impl PublicKey {
    pub fn encrypt(&self, m: &BigInt) -> BigInt{
        let mut r;


        let mut rng = match StdRng::new() {
            Ok(g) => g,
            Err(e) => panic!("Failed to obtain OS RNG: {}", e)
        };

        r = rng.gen_bigint_range(&self.n, &self.n_squared);

        let mut result = self.g.mod_pow(m, &self.n_squared);
        let x = r.mod_pow(&self.n, &self.n_squared);

        result = result * x;
        result = result % &self.n_squared;

        result
    }
}
