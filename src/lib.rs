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

#[macro_export]
macro_rules! make_solutions {
    ($name:expr, [$s1:ident, $($ss:ident),* $(,)?]) => {{
        let args = clap::Command::new($name)
            .arg(
                clap::Arg::new("solution")
                    .default_value(stringify!($s1))
                    .value_parser(name_array![$s1, $($ss),*])
            )
            .get_matches();
        if let Some(s) = args.get_one::<String>("solution") {
            match s.as_str() {
                stringify!($s1) => println!("{}", $s1()),
                $(stringify!($ss) => println!("{}", $ss()),)*
                _ => unreachable!(),
            }
        } else {
            unreachable!();
        }
    }};
}

#[macro_export]
macro_rules! name_array {
    [$($list:ident),* $(,)?] => {
        [$(stringify!($list)),*]
    }
}
