extern crate rand;
use rand::Rng;
use std::u64;

struct keys {
    pub_key_1: u64,
    pub_key_2: u32,
    priv_key: u32
}


fn generate(keysize: u32) -> keys {
    let e: u32 = 2u32.pow(63);
    let mut p: u32;
    let mut q: u32;
    let mut lambda: u32;

    while {
        p = randomPrime(keysize/2);
        q = randomPrime(keysize/2);
        lambda = lcm(p-1, q-1);

        gcd(e, lambda) != 1 || ((p-q) as i64).abs() >= 2i64.pow(keysize/2 - 100)
    } {}

    return keys {pub_key_1: (p*q) as u64, pub_key_2: e, priv_key: (1/e % lambda) }

}

fn encrypt(message: u32, pub_key_1: u64, pub_key_2: u32) -> u64 {
    return message.pow(pub_key_2) as u64 % pub_key_1;
}

fn decrypt(message: u64, priv_key: u32, pub_key_1: u64) -> u64 {
    return message.pow(priv_key) as u64 % pub_key_1;
}

fn randomPrime(bits: u32) -> u32{
    /*
    Generates a random k-bit prime greater than sqrt(2)*2^(k-1)
     */
    let min: f32 = 2f32.sqrt()*(2u32.pow(bits-1) as f32);

    let a: u32 = 2u32.pow(bits) - 1;
    let max: f32 = a as f32;

    let mut rng = rand::thread_rng();
    let p = rng.gen_range(min, max) as u32;

    if isPrime(p) {
        return p;
    } else {
        return p-1;
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    let mut t: u32;

    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }

    return a;
}

fn lcm(a: u32, b: u32) -> u32 {
    return (a*b)/gcd(a, b);
}

fn isPrime(number: u32) -> bool {

    if number <= 1 {
        return false;
    } else if number <= 3 {
        return true;
    } else if number % 2 == 0 || number % 3 == 0 {
        return false;
    } else {
        let mut i = 5;
        while i*i <= number {
            if number % i == 0 || number % (i+2) == 0 {
                return false;
            }
            i  = i+6;
        }
        return true;
    }
}
