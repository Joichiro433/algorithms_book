fn exhaustive_search(nums: Vec<i32>, target: i32) -> bool {
    for bit in 0..(1<<nums.len()) {
        let mut total: i32 = 0;
        for (idx, num) in nums.iter().enumerate() {
            if bit & (1<<idx) > 0 {
                total += *num;
            }
        }
        if total == target {
            return true;
        }
    }
    false
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 4, 5, 11];
    let target: i32 = 10;
    let result: bool = exhaustive_search(nums, target);

    println!("{}", result);
}