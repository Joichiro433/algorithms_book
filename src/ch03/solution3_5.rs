use std::cmp::min;


fn get_div_two_times(mut num: i32) -> i32 {
    let mut times: i32 = 0;
    while num%2 == 0 {
        num /= 2;
        times += 1;
    }
    times
}

fn main() {
    let nums: Vec<i32> = vec![8, 12, 40];
    let mut result = 200000;
    for num in nums {
        result = min(result, get_div_two_times(num));
    }

    println!("{}", result);
}