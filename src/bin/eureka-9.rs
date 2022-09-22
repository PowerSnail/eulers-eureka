use eulers_eureka::make_solutions;

fn naive() -> u64 {
    for c in 3..=1000 {
        for b in (c + 1) / 2..c {
            let a = 1000 - c - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    unreachable!();
}

fn main() {
    make_solutions!("Eureka 9", [naive]);
}
