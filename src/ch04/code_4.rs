fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fibonacci(n-1) + fibonacci(n-2)
}


fn main() {
    let result: i32 = fibonacci(10);
    println!("{}", result);
}