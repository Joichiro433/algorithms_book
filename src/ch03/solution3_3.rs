fn search_second_min_val(nums: Vec<i32>) -> i32 {
    let mut minimum_val: i32 = 200000;
    let mut second_val: i32 = 200000;
    for num in nums {
        if num < minimum_val {
            second_val = minimum_val;
            minimum_val = num;
        } else if num < second_val {
            second_val = num;
        }
    }
    second_val
}

fn main() {
    let nums: Vec<i32> = vec![3, 4, 1, 2, 5, 8];
    let result: i32 = search_second_min_val(nums);

    println!("{}", result);
}