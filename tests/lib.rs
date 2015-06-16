extern crate num;
extern crate rpaillier;

use num::bigint::{ BigInt, Sign };
use num::traits::FromPrimitive;
use rpaillier::bigint_extensions::{ Two, Three, ModInverse, ModPow};

use rpaillier::{ KeyPair, PublicKey, KeyPairBuilder };

#[test]
fn test_bigint_two() {
    let a = BigInt::two();
    let e = BigInt::from_u8(2).unwrap();
    assert_eq!(e, a);
}

#[test]
fn test_bigint_three() {
    let a = BigInt::three();
    let e = BigInt::from_u8(3).unwrap();
    assert_eq!(e, a);
}

#[test]
fn test_bigint_mod_pow() {
    fn check(b: &BigInt, e: &BigInt, m: &BigInt, r: &BigInt) {
        assert_eq!(b.mod_pow(&e, &m), *r);
    }

    fn check_i64(b: i64, e: i64, m: i64, r: i64) {
        let big_b = BigInt::from_i64(b).unwrap();
        let big_e = BigInt::from_i64(e).unwrap();
        let big_m = BigInt::from_i64(m).unwrap();
        let big_r = BigInt::from_i64(r).unwrap();

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

    let big_b = BigInt::new(Sign::Plus, vec![12,234,234,556,34]);
    let big_e = BigInt::new(Sign::Plus, vec![12,234,234,556,34]);
    let big_m = BigInt::new(Sign::Plus, vec![234,556,34]);
    let big_r = BigInt::new(Sign::Plus, vec![2689017340, 2002504038, 5]);

    check(&big_b, &big_e, &big_m, &big_r);
}

#[test]
fn test_bigint_mod_inverse() {
    fn check(a: i64, b: i64, c: i64) {
        let big_a = BigInt::from_i64(a).unwrap();
        let big_b = BigInt::from_i64(b).unwrap();
        let big_c = BigInt::from_i64(c).unwrap();

        assert_eq!(big_a.mod_inverse(&big_b).unwrap(), big_c);
    }

    fn check_none(a: i64, b: i64) {
        let big_a = BigInt::from_i64(a).unwrap();
        let big_b = BigInt::from_i64(b).unwrap();

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

    let m = BigInt::from_u8(37).unwrap();

    let c = public_key.encrypt(&m);
    let a = key_pair.decrypt(&c);

    assert_eq!(m, a);
}

#[test]
fn test_homomorphic_properties() {
    let kp = KeyPairBuilder::new().bits(128).finalize();

    let pk = &kp.public_key;

    let m1 = BigInt::from_u8(37).unwrap();
    let m2 = BigInt::from_u8(37).unwrap();

    let c1 = pk.encrypt(&m1);
    let c2 = pk.encrypt(&m2);

    let add = (c1 * c2) % &pk.n_squared;

    let e = m1 + m2;
    let a = kp.decrypt(&add);

    assert_eq!(a, e);
}
