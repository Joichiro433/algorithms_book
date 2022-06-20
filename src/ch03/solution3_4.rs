fn search_max_diff(nums: Vec<i32>) -> i32 {
    let mut min_val: i32 = 200000;
    let mut max_val: i32 = -200000;
    for num in nums {
        if num < min_val {
            min_val = num;
        }
        if num > max_val {
            max_val = num;
        }
    }
    max_val - min_val
}

fn main() {
    let nums: Vec<i32> = vec![3, 4, 1, 2, 5, 8];
    let result: i32 = search_max_diff(nums);

    println!("{}", result);
}