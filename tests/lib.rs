extern crate ramp;
extern crate rpaillier;

use ramp::int::{ Int };

use rpaillier::KeyPairBuilder;

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
