pub fn primes_under(n: u64) -> Vec<u64> {
    let mut sieve = vec![true; n as usize];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..sieve.len() {
        if sieve[i] {
            for j in ((i + i)..sieve.len()).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    sieve
        .into_iter()
        .enumerate()
        .filter(|(_, is_prime)| *is_prime)
        .map(|(i, _)| i as u64)
        .collect()
}
