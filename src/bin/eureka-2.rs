use eulers_eureka::make_solutions;

fn naive() -> u64 {
    let mut sum: u64 = 0;
    let mut x: u64 = 1;
    let mut y: u64 = 2;
    while y <= 4_000_000 {
        if y % 2 == 0 {
            sum += y;
        }
        let x_ = x + y;
        x = y;
        y = x_;
    }
    sum
}

fn count_3() -> u64 {
    let mut buffer = [1, 1, 2];
    let mut sum = 0;
    while buffer[2] < 4_000_000 {
        sum += buffer[2];
        buffer[0] = buffer[1] + buffer[2];
        buffer[1] = buffer[2] + buffer[0];
        buffer[2] = buffer[0] + buffer[1];
    }
    sum
}

fn main() {
    make_solutions!("Eureka 2", [count_3, naive]);
}
