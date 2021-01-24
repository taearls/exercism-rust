use rand::Rng;
use num::BigInt;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    // generate a random number within half-open range 2 to p
    rng.gen_range(2..p)
}
// public key A = g**a mod p
// pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
//     modular_exponentiation_alt(g, a, p)
// }

// pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
//     modular_exponentiation_alt(b_pub, a, p)
// }

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let pub_key = BigInt::from(g).modpow(&BigInt::from(a), &BigInt::from(p));
    pub_key.to_str_radix(10).parse::<u64>().unwrap()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let priv_key = BigInt::from(b_pub).modpow(&BigInt::from(a), &BigInt::from(p));
    priv_key.to_str_radix(10).parse::<u64>().unwrap()
}

// formula based on memory-efficient method found here: 
// https://en.wikipedia.org/wiki/Modular_exponentiation 
fn modular_exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut loop_iterations: u64 = 0;
    let mut result: u64 = 1;

    // if exponent == 0, product should be 1
    if exponent > 0 {
        loop {
            result = (result * base) % modulus;
            loop_iterations += 1;
            if loop_iterations == exponent {
                break;
            }
        }
    }
    result
}
// forula based on Right-to-left binary method found here: 
// https://en.wikipedia.org/wiki/Modular_exponentiation 
fn modular_exponentiation_alt(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result: u64 = 1;

    let mut temp_exponent = exponent;
    let mut temp_base = base % modulus;

    loop {
        if temp_exponent == 0 {
            break;
        } else if temp_exponent % 2 == 1 {
            result = (result * temp_base) % modulus;
        }
        temp_exponent = temp_exponent >> 1;
        temp_base = (temp_base.pow(2)) % modulus;
    }
    result
}
