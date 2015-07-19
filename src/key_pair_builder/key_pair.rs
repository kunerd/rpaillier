use ramp::Int;
use bigint_extensions::ModPow;

use public_key::PublicKey;
use private_key::PrivateKey;

pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: PrivateKey
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
