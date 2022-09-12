const TARGET: usize = 600851475143;

fn sieved() -> usize {
    let mut sieve = vec![true; (TARGET as f32).sqrt() as usize + 1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..sieve.len() {
        if sieve[i] {
            for j in ((i + i)..sieve.len()).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    for i in (2..sieve.len()).rev() {
        if sieve[i] && TARGET % i == 0 {
            return i;
        }
    }
    unreachable!("Not found?!");
}
fn main() {
    println!("{}", sieved());
}
