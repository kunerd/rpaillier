// use core::ops::{ Mul, Rem };
use core::ops::Mul;

use num::traits::{ Zero, One, FromPrimitive, ToPrimitive };
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

const TABLE_BASE: usize = 5;

impl ModPow<BigInt> for BigInt {

    // Left-to-right k-ary exponentiation
    fn mod_pow(&self, exp: &BigInt, m: &BigInt) -> BigInt {

        let base = 2 << (TABLE_BASE - 1);

        let mut table = Vec::with_capacity(base);
        table.push(BigInt::one());

        for i in 1..base {
            let last = table.get(i-1).unwrap().clone();

            table.push((last * self) % m);
        }

        let mut r = BigInt::one();

        for i in digits_of_n(exp, base).iter().rev() {
            for _ in 0..TABLE_BASE {
                r = &r * &r % m
            }

            if (*i) != 0 {
                r = r * table.get((*i)).unwrap() % m;
            }
        }

        r
    }
}

fn digits_of_n(e: &BigInt, b: usize) -> Vec<usize> {
    let mut digits = Vec::new();

    let mut n = (*e).clone();
    let base = &BigInt::from_usize(b).unwrap();
    while n > BigInt::zero() {
        digits.push((&n % base).to_usize().unwrap());
        n = &n / base;
    }

    return digits
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
