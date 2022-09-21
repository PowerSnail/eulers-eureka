use eulers_eureka::make_solutions;

const MAX: u64 = 100;

fn naive() -> u64 {
    let sum_of_sq: u64 = (1..=MAX).map(|x| x * x).sum();
    let sq_of_sum: u64 = (1..=MAX).sum::<u64>().pow(2);
    sq_of_sum - sum_of_sq
}

fn better() -> u64 {
    let sum = (1 + MAX) * MAX / 2;
    (1..=MAX).map(|i| i * (sum - i)).sum()
}

fn main() {
    make_solutions!("Eureka 6", [better, naive]);
}
