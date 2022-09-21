use eulers_eureka::make_solutions;

fn is_pan(n: u64) -> bool {
    let digits: Vec<u64> = (0..6)
        .map(|i| 10u64.pow(i))
        .map(|b| n / b)
        .take_while(|&d| d != 0)
        .map(|n| n % 10)
        .collect();
    for i in 0..digits.len() / 2 {
        if digits[i] != digits[digits.len() - 1 - i] {
            return false;
        }
    }
    true
}

fn naive() -> u64 {
    let mut max = 0u64;
    for i in (100..1000).rev() {
        for j in (100..(i + 1)).rev() {
            if is_pan(i * j) {
                max = (i * j).max(max);
                break;
            }
        }
    }
    max
}

fn main() {
    make_solutions!("Eureka 4", [naive,]);
}
