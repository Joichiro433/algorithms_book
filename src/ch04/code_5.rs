fn fibonacci(n: usize, memo: &mut Vec<i32>) -> i32 {
    if memo[n] != -1 {
        return memo[n];
    }
    memo[n] = fibonacci(n-1, memo) + fibonacci(n-2, memo);
    memo[n]
}


fn main() {
    let n = 35;
    let mut memo = vec![-1; n+1];
    memo[0] = 0;
    memo[1] = 1;
    let result = fibonacci(n, &mut memo);

    println!("{}", result);

}