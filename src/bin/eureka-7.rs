use eulers_eureka::{primes_under, make_solutions};


const TARGET: u64 = 10001;

fn sieve() -> u64 {
    let mut max = TARGET * 2;
    loop {
        let primes = primes_under(max);
        if primes.len() < TARGET as usize{
            max *= 2;
            continue;
        }
        return primes[TARGET as usize - 1];
    }
}

fn main() {
    make_solutions!("Eureka 7", [sieve])
}