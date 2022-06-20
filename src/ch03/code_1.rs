fn linear_search(nums: Vec<i32>, target: i32) -> bool {
    for num in nums {
        if num == target {
            return true;
        }
    }
    false
}


fn main() {
    let nums: Vec<i32> = vec![3, 4, 1, 2, 5, 7];
    let target: i32 = 5;
    let result: bool = linear_search(nums, target);

    println!("{}", result);
}