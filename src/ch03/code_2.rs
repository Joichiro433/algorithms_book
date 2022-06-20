fn linear_search(nums: Vec<i32>, target: i32) -> i32 {
    for (idx, num) in nums.iter().enumerate() {
        if *num == target {
            return idx as i32;
        }
    }
    -1
}


fn main() {
    let nums: Vec<i32> = vec![3, 4, 1, 2, 5, 7];
    let target: i32 = 5;
    let result: i32 = linear_search(nums, target);

    println!("{}", result);
}