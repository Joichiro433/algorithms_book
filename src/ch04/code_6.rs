fn exist_if_sum_pair(i: usize, target: i32, nums: &Vec<i32>) -> bool {
    if i == 0 {
        return target == 0;
    }
    if exist_if_sum_pair(i-1, target, nums) {
        return true;
    }
    if exist_if_sum_pair(i-1, target - nums[i-1], nums) {
        return true;
    }
    false
}


fn main() {
    let target: i32 = 10;
    let nums: Vec<i32> = vec![1, 2, 4, 5, 11];
    let result: bool = exist_if_sum_pair(nums.len(), target, &nums);

    println!("{}", result);
}