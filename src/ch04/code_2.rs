fn recursive_func(n: i32) -> i32 {
    println!("func({})を呼び出しました", n);
    if n == 0 {
        return 0;
    }
    let result: i32 = n + recursive_func(n-1);
    println!("{} までの和 = {}", n, result);
    result
}


fn main() {
    recursive_func(5);
}