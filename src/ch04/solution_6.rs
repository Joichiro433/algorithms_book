fn exist_if_sum_pair(i: usize, target: usize, nums: &Vec<usize>, memo: &mut Vec<Vec<i32>>) -> bool {
    if i == 0 {
        return target == 0;
    }
    if memo[i][target] != -1 {
        match memo[i][target] {
            0 => return false,
            _ => return true,
        }
    }
    if exist_if_sum_pair(i-1, target, nums, memo) {
        memo[i][target] = 1;
        return true
    }
    if exist_if_sum_pair(i-1, target-nums[i-1], nums, memo) {
        memo[i][target] = 1;
        return true
    }
    memo[i][target] = 0;
    false
}


fn main() {
    let target: usize = 10;
    let nums: Vec<usize> = vec![1, 2, 4, 5, 11];
    let n: usize = nums.len(); 
    let mut memo: Vec<Vec<i32>> = vec![vec![-1; target+1]; n+1];

    let result: bool = exist_if_sum_pair(n, target, &nums, &mut memo);

    println!("{}", result);
}