fn tribonacci(n: usize, memo: &mut Vec<i32>) -> i32 {
    if memo[n] != -1 {
        return memo[n];
    }
    memo[n] = tribonacci(n-1, memo) + tribonacci(n-2, memo) + tribonacci(n-3, memo);
    memo[n]
}


fn main() {
    let n: usize = 10;
    let mut memo: Vec<i32> = vec![-1; n+1];
    memo[0] = 0;
    memo[1] = 0;
    memo[2] = 1;
    let result: i32 = tribonacci(n, &mut memo);

    println!("{}", result);
}