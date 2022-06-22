fn tribonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 0,
        2 => 1,
        _ => tribonacci(n-1) + tribonacci(n-2) + tribonacci(n-3),
    }
}


fn main() {
    let result: i32 = tribonacci(10);
    println!("{}", result);
}