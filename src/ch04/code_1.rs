fn recursive_func(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    n + recursive_func(n-1)
}


fn main() {
    let result: i32 = recursive_func(5);
    println!("{}", result);
}