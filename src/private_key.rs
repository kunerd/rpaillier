use num::bigint::BigInt;

pub struct PrivateKey {
    pub lambda: BigInt,
    pub denominator: BigInt
}
