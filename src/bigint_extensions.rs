use ramp::int::Int;
use core::convert::From;

const DEFAULT_BUCKET_SIZE: usize = 5;

pub trait ModPow<T, K> {
    fn mod_pow(&self, exp: &T, m: &K) -> Self;
    fn mod_pow_k(&self, exp: &T, m: &K, k: usize) -> Self;
}

impl ModPow<Int, Int> for Int {

    fn mod_pow(&self, exp: &Int, m: &Int) -> Int {
        self.mod_pow_k(exp, m, DEFAULT_BUCKET_SIZE)
    }

    fn mod_pow_k(&self, exp: &Int, m: &Int, k: usize) -> Int {

        let base = 2 << (k - 1);

        let mut table = Vec::with_capacity(base);
        table.push(Int::one());

        for i in 1..base {
            let last = table.get_mut(i-1).unwrap().clone();

            table.push((last * self) % m);
        }

        let mut r = Int::one();

        for i in digits_of_n(exp, base).iter().rev() {
            for _ in 0..k {
                r = &r * &r % m
            }

            if *i != 0 {
                r = &r * table.get(*i).unwrap() % m;
            }
        }

        r
    }
}

fn digits_of_n(e: &Int, b: usize) -> Vec<usize> {
    let mut digits = Vec::new();

    let mut n = (*e).clone();
    let base = Int::from(b);

    while n > Int::zero() {
        digits.push(usize::from(&(&n % &base)));
        n = &n / &base;
    }

    digits
}


pub trait ModInverse<T> {
    fn mod_inverse(&self, n: &T) -> Option<Self>;
}

impl ModInverse<Int> for Int {

    fn mod_inverse(&self, n: &Int) -> Option<Int> {
        let mut u1 = Int::one();
        let mut u3 = (*self).clone();
        let mut v1 = Int::zero();
        let mut v3 = (*n).clone();

        let mut iter = true;

        while v3 != Int::zero()
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

        if u3 != Int::one() {
            return None;
        }

        let inv = if iter == false {
            n - u1
        } else {
            u1
        };

        Some(inv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ramp::{ Int, RandomInt };

    use rand;

    use test::Bencher;

    #[test]
    fn test_bigint_mod_pow() {
        fn check(b: &Int, e: &Int, m: &Int, r: &Int) {
            assert_eq!(b.mod_pow(&e, &m), *r);
        }

        fn check_i64(b: i64, e: i64, m: i64, r: i64) {
            let big_b = Int::from(b);
            let big_e = Int::from(e);
            let big_m = Int::from(m);
            let big_r = Int::from(r);

            check(&big_b, &big_e, &big_m, &big_r);
        }


        check_i64(-2, 5, 33, -32);
        check_i64(-2, 5, 32, 0);
        check_i64(-1, 3, 10, -1);
        check_i64(-1, 4, 10, 1);
        check_i64(0, 2352, 21, 0);
        check_i64(1, 26, 21, 1);
        check_i64(2, 5, 33, 32);
        check_i64(2, 5, 32, 0);
        check_i64(::std::i64::MAX, ::std::i64::MAX, 2, 1);
    }

    #[test]
    fn test_bigint_mod_inverse() {
        fn check(a: i64, b: i64, c: i64) {
            let big_a = Int::from(a);
            let big_b = Int::from(b);
            let big_c = Int::from(c);

            assert_eq!(big_a.mod_inverse(&big_b).unwrap(), big_c);
        }

        fn check_none(a: i64, b: i64) {
            let big_a = Int::from(a);
            let big_b = Int::from(b);

            assert_eq!(big_a.mod_inverse(&big_b), None);

        }

        check(7, 26, 15);
        check(37, 216, 181);
        check(17, 3120, 2753);
        check(7, -72, 31);
        check_none(0, 21);
        check_none(0, 198);
        check_none(7, 21);
    }

    #[bench]
    fn bench_mod_pow(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        let base = rng.gen_uint(265);
        let m = rng.gen_uint(265);

        b.iter(|| {
            let exp = rng.gen_uint(265);

            base.mod_pow(&exp, &m);
        });
    }

    #[bench]
    fn bench_mod_inverse(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = rng.gen_uint(128);

        b.iter(|| {
            let a = rng.gen_uint(128);

            a.mod_inverse(&m);
        });
    }
}
