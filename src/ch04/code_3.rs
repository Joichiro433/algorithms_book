fn euclidean_algorithm(m: i32, n: i32) -> i32 {
    if n == 0 {
        return m;
    }
    euclidean_algorithm(n, m%n)
}


fn main() {
    let result: i32 = euclidean_algorithm(51, 15);
    println!("{}", result);
}