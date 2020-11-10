fn main() {
    println!("20F -> {}C", to_c(20.0));
    println!("68F -> {}C", to_f(68.0));

    for i in 1..50 {
        println!("fib({}) -> {}", i, fib(i));
    }
}

const BASE: f64 = 32.0;
const FACTOR: f64 = 9.0 / 5.0;

fn to_c(temp_c: f64) -> f64 {
    temp_c * FACTOR + BASE
}

fn to_f(temp_f: f64) -> f64 {
    (temp_f - BASE) / FACTOR
}

fn fib(n: i32) -> i64 {
    if 0 == n {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
