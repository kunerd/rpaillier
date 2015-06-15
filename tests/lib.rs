extern crate num;
extern crate rpaillier;

use num::bigint::BigInt;
use num::traits::FromPrimitive;
use rpaillier::bigint_extensions::{ Two, Three, ModInverse};

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
