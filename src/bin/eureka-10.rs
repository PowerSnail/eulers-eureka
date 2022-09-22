use eulers_eureka::{primes_under, make_solutions};

const TARGET: u64 = 2_000_000;
// const TARGET: u64 = 10;


fn sieve() -> u64 {
    primes_under(TARGET + 1).into_iter().sum()
}


fn main() {
    make_solutions!("Eureka 10", [sieve]);
}