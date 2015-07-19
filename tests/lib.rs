extern crate ramp;
extern crate rpaillier;

use ramp::int::{ Int };
use rpaillier::bigint_extensions::{ ModPow, ModInverse };

use rpaillier::{ KeyPair, PublicKey, KeyPairBuilder };

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
    check_i64(std::i64::MAX, std::i64::MAX, 2, 1);
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

#[test]
fn test_encrypt_decrypt() {
    let key_pair = KeyPairBuilder::new().bits(128).finalize();

    let public_key = &key_pair.public_key;

    let m = Int::from(37);

    let c = public_key.encrypt(&m);
    let a = key_pair.decrypt(&c);

    assert_eq!(m, a);
}

#[test]
fn test_homomorphic_properties() {
    let kp = KeyPairBuilder::new().bits(128).finalize();

    let pk = &kp.public_key;

    let m1 = Int::from(37);
    let m2 = Int::from(132);

    let c1 = pk.encrypt(&m1);
    let c2 = pk.encrypt(&m2);

    let add = (c1 * c2) % &pk.n_squared;

    let e = m1 + m2;
    let a = kp.decrypt(&add);

    assert_eq!(a, e);
}
