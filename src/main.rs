use num_bigint::BigUint;
use primal::Primes;
use num_traits::{Zero, One};
use std::time::Instant;

fn main() {
    // Replace this with the number you want to factorize
    let number_str = "21888242871839275222246405745257275088696311157297823662689037894645226208587";
    let mut number = BigUint::parse_bytes(number_str.as_bytes(), 10).unwrap();
    let mut factors = Vec::new();
    let primes = Primes::all();
    
    let start = Instant::now();
    let mut count = 0;

    for prime in primes {
        let prime_biguint = BigUint::from(prime);
        while &number % &prime_biguint == BigUint::zero() {
            factors.push(prime_biguint.clone());
            number /= &prime_biguint;
        }
        if number == BigUint::one() {
            break;
        }

        // Print progress every 1000 primes
        if count % 1000 == 0 {
            println!("Checked up to prime: {}", prime);
        }
        count += 1;
    }

    if number != BigUint::one() {
        factors.push(number.clone());
    }
    
    println!("Factors: {:?}", factors);
    println!("Time taken: {:?}", start.elapsed());
}
