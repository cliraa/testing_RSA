use num_traits::FromPrimitive;
use num_bigint::BigInt;
use rand::Rng;
use rand::thread_rng;

fn generate_random() -> BigInt {
    let mut rng = rand::thread_rng();
    let random_number: u64 = rng.gen();
    let big_number: BigInt = FromPrimitive::from_u64(random_number).unwrap();
    let random_biguint: BigInt = big_number << 256;
    random_biguint
}

fn miller_rabin(n: &BigInt, k: usize) -> String {
    if n <= &BigInt::from(1) {
        return "composite".to_string();
    }

    let mut d = n - BigInt::from(1);
    let mut s = 0;

    while &d % 2 == BigInt::from(0) {
        d /= BigInt::from(2);
        s += 1;
    }

    let mut rng = thread_rng();

    for _ in 0..k {
        let a = rng.gen_range(BigInt::from(1)..n.clone());
        let mut x = a.modpow(&d, &n);

        if x == BigInt::from(1) || x == n - BigInt::from(1) {
            continue;
        }

        for _ in 0..s - 1 {
            x = x.modpow(&BigInt::from(2), &n);

            if x == BigInt::from(1) {
                return "composite".to_string();
            }

            if x == n - BigInt::from(1) {
                break;
            }
        }

        if x != n - BigInt::from(1) {
            return "composite".to_string();
        }
    }

    "probably prime".to_string()
}

fn main() {
    let mut p = generate_random();

    while miller_rabin(&p, 10) != "probably prime" {
        p = generate_random();
    }

    let mut q = generate_random();

    while miller_rabin(&q, 10) != "probably prime" || q == p {
        q = generate_random();
    }

    println!("p: {}", p);
    println!("q: {}", q);
}
