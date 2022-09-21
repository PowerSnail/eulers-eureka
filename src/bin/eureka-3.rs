use eulers_eureka::{primes_under, make_solutions};

const TARGET: usize = 600851475143;


fn sieved() -> u64 {
    primes_under((TARGET as f32).sqrt() as u64 + 1)
        .into_iter()
        .rev()
        .filter(|n| TARGET as u64 % n == 0)
        .next()
        .unwrap()
}

fn main() {
    make_solutions!("Eureka 3", [sieved]);
}
