# rPaillier
A Rust implementation of the Paillier cryptosystem

usage
-----

create a key pair:
```
    let key_pair = KeyPairBuilder::new().bits(128)
                                        .finalize();
```
encryption:
```
    let public_key = &key_pair.public_key;
    let c = public_key.encrypt(&m);
```

decrypt a ciphertext:
```
    let m = key_pair.decrypt(&c);
```


##References
==========
 * [Public-Key Cryptosystems Based on Composite
Degree Residuosity Classes](http://www.cs.tau.ac.il/~fiat/crypt07/papers/Pai99pai.pdf)
 * [Wikipedia article](https://en.wikipedia.org/wiki/Paillier_cryptosystem)
