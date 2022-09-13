use eulers_eureka::primes_under;

fn naive() -> u64 {
    for n in 21.. {
        if (2..=20).all(|d| n % d == 0) {
            return n;
        }
    }
    unreachable!();
}

fn prime_factors() -> u64 {
    println!("{:?}", primes_under(11));
    primes_under(11)
        .into_iter()
        .reduce(|x, y| x * y)
        .unwrap()
}

fn main() {
    // println!("{}", naive());
    println!("{}", prime_factors());
}