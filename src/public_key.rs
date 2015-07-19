use ramp::{ Int, RandomInt };
use rand::StdRng;
use bigint_extensions::ModPow;


pub struct PublicKey {
    pub bits: usize,
    pub n: Int,
    pub n_squared: Int,
    pub g: Int
}

impl PublicKey {
    pub fn encrypt(&self, m: &Int) -> Int{
        let mut r;

        let mut rng = match StdRng::new() {
            Ok(g) => g,
            Err(e) => panic!("Failed to obtain OS RNG: {}", e)
        };

        r = rng.gen_uint_range(&self.n, &self.n_squared);

        let mut result = self.g.mod_pow(m, &self.n_squared);
        let x = r.mod_pow(&self.n, &self.n_squared);

        result = result * x;
        result = result % &self.n_squared;

        result
    }
}
