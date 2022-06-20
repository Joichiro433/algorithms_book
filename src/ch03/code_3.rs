fn search_min_val(nums: Vec<i32>) -> i32 {
    let mut min_val: i32 = nums[0];
    for num in nums {
        if num < min_val {
            min_val = num;
        }
    }
    min_val
}


fn main() {
    let nums: Vec<i32> = vec![3, 4, 1, 2, 5, 7];
    let result: i32 = search_min_val(nums);

    println!("{}", result);
}