fn count_num(nums: Vec<i32>, target: i32) -> i32 {
    let mut counter: i32 = 0;
    for num in nums {
        if num == target {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let nums: Vec<i32> = vec![3, 4, 1, 2, 5, 1];
    let target: i32 = 1;
    let result: i32 = count_num(nums, target);

    println!("{}", result);
}