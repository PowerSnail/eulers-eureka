fn naive() -> u32 {
    (0..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

fn gauss_sum(start: u32, end: u32, count: u32) -> u32{
    (start + end) * count / 2
}

fn solution_gauss_sum() -> u32 {
    gauss_sum(3, 999 / 3 * 3, 999 / 3) 
    + gauss_sum(5, 999 / 5 * 5, 999 / 5) 
    - gauss_sum(15, 999 / 15 * 15, 999 / 15)
}

fn main() {
    // println!("{}", naive());
    println!("{}", solution_gauss_sum());
}
