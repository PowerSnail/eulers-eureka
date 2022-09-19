use eulers_eureka::primes_under;

const TARGET: u64 = 20;

fn naive() -> u64 {
    for n in (TARGET+1).. {
        if (2..=TARGET).all(|d| n % d == 0) {
            return n;
        }
    }
    unreachable!();
}

fn larget_exponent(max: u64, base: u64) -> u64 {
    base.pow((max as f32).log(base as f32).floor() as u32)
}

fn prime_factors() -> u64 {
    primes_under(TARGET+1)
        .into_iter()
        .map(|n| larget_exponent(TARGET, n))
        .reduce(|x, y| x * y)
        .unwrap()
}

fn main() {
    // println!("{}", naive());
    println!("{}", prime_factors());
}