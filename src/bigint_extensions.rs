use core::ops::{ Mul, Rem };

use num::traits::{ Zero, One };
use num::integer::Integer;
use num::bigint::{ BigInt, Sign };

pub trait Two: Mul<Self, Output=Self> {
    fn two() -> Self;
}

impl Two for BigInt {
    #[inline]
    fn two() -> BigInt {
        BigInt::new(Sign::Plus, vec!(2))
    }
}

pub trait Three: Mul<Self, Output=Self> {
    fn three() -> Self;
}

impl Three for BigInt {
    #[inline]
    fn three() -> BigInt {
        BigInt::new(Sign::Plus, vec!(3))
    }
}

pub trait ModPow<T> {
    fn mod_pow(&self, exp: &T, m: &T) -> T;
}

impl ModPow<BigInt> for BigInt {
    #[no_mangle]
    fn mod_pow(&self, exp: &BigInt, m: &BigInt) -> BigInt {
        let mut a = self.rem(m);
        let mut b = (*exp).clone();
        let mut p = BigInt::one();

        while b > BigInt::zero() {
             if b.is_odd() {
                 p = p * &a;
                 p = p % m;
             }
             b = b / BigInt::two();
             a = (&a * &a) % m;
        }

        p
    }
}

pub trait ModInverse<T> {
    fn mod_inverse(&self, n: &BigInt) -> Option<BigInt>;
}

impl ModInverse<BigInt> for BigInt {

    fn mod_inverse(&self, n: &BigInt) -> Option<BigInt> {
        let mut u1 = BigInt::one();
        let mut u3 = (*self).clone();
        let mut v1 = BigInt::zero();
        let mut v3 = (*n).clone();

        let mut iter = true;

        while (v3 != BigInt::zero())
        {
            let q = &u3 / &v3;
            let t3 = u3 % &v3;
            let t1 = u1 + &q * &v1;

            u1 = v1.clone();
            v1 = t1.clone();
            u3 = v3.clone();
            v3 = t3.clone();

            iter = !iter;
        }

        if u3 != BigInt::one() {
            return None;
        }

        let inv = if (iter == false) {
            n - u1
        } else {
            u1
        };

        Some(inv)
    }
}

#[cfg(test)]
mod tests {

}
