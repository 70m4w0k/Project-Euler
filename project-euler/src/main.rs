fn main() {
    let target = 999;
    let result = sum_divisible_by(3, target) + sum_divisible_by(5, target) - sum_divisible_by(15, target);

    println!("{}", result);
}

fn sum_divisible_by(n:i64, target: i64) -> i64 {
    let p = target / n;
    return n*(p*(p + 1)) / 2
}
