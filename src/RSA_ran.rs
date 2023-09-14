use num_traits::FromPrimitive;
use num_traits::{One, Zero};
use num_integer::Integer;
use num_bigint::BigInt;
use num_bigint::Sign;
use glass_pumpkin::prime;

fn main() {

    // Step 1: Choose two prime numbers p and q: 
    //let p = BigInt::parse_bytes(b"115792089237316195423570985008687907853269984665640564039457584007908834671663", 10).unwrap();
    //let q = BigInt::parse_bytes(b"17", 10).unwrap();

    let no_bits=256;
    let  p = BigInt::from_biguint(Sign::Plus,prime::new(no_bits).unwrap());
    let  q = BigInt::from_biguint(Sign::Plus,prime::new(no_bits).unwrap());

    println!("p: {}", p);
    println!("q: {}", q);

    // Step 2: Compute n = pq:
    let n: BigInt = p.clone() * q.clone();
    println!("n: {}", n);

    // Step 3: Compute λ(n) = lcm(p-1, q-1):
    let lambda_n = lcm(&(p.clone() - BigInt::one()), &(q.clone() - BigInt::one()));
    println!("The lambda_n's value is: {}", lambda_n);

    // Step 4: Compute e:
    let e = BigInt::parse_bytes(b"65537", 10).unwrap();
    println!("e: {}", e);

    // Step 5: Compute d as the modular multiplicative inverse of e (mod lambda_n):
    let d = modular_inverse(&e.clone(), &lambda_n);    
    println!("d: {}", d);

    // Step 6: Message:
    let m: BigInt = num_bigint::BigInt::from_u64(65).unwrap();
    println!("m: {}", m);

    // Step 7: Encryption:
    let encrypted = mod_pow(m, e, n.clone());
    println!("encrypted_message: {}", encrypted);

    // Step 8: Decryption:
    let decrypted = mod_pow(encrypted, d, n);
    println!("decrypted_message: {}", decrypted);

}

fn lcm(p: &BigInt, q: &BigInt) -> BigInt {
    if p.is_zero() || q.is_zero() {
        return BigInt::zero();
    }

    let gcd = euclidean_algorithm(p.clone(), q.clone());
    (p / &gcd) * q
}

fn euclidean_algorithm(a: BigInt, b: BigInt) -> BigInt {
    let mut dividend = a.clone();
    let mut divisor = b.clone();

    while !divisor.is_zero() {
        let remainder = dividend % divisor.clone();
        dividend = divisor;
        divisor = remainder;
    }

    dividend
}

fn modular_inverse(n: &BigInt, p: &BigInt) -> BigInt {
    if p.is_one() { return BigInt::one() }

    let (mut a, mut m, mut x, mut inv) = (n.clone(), p.clone(), BigInt::zero(), BigInt::one());

    while a > BigInt::one() {
        let (div, rem) = a.div_rem(&m);
        inv -= div * &x;
        a = rem;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x, &mut inv);
    }
 
    if inv < BigInt::zero() { inv += p }

    inv
}

fn mod_pow(base: BigInt, exp: BigInt, modulus: BigInt) -> BigInt {
    if modulus.is_one() {
        return BigInt::zero();
    }

    let mut result = BigInt::one();
    let mut base = base % &modulus;
    let mut exp = exp.clone();

    while exp > BigInt::zero() {
        if exp.is_odd() {
            result = (&result * &base) % &modulus;
        }

        exp = exp >> 1;
        base = (&base * &base) % &modulus;
    }

    result
}
