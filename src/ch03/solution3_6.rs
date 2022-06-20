use std::cmp::min;


fn get_three_pattern_times(target: i32, threshold: i32) -> i32 {
    let mut counter: i32 = 0;
    for x in 0..=min(target, threshold) {
        for y in 0..=min(target, threshold) {
            let z: i32 = target - (x + y);
            if 0 <= z && z <= threshold {
                counter += 1;
            }
        }
    }
    counter
}


fn main() {
    let target: i32 = 4;
    let threshold: i32 = 5;
    let result: i32 = get_three_pattern_times(target, threshold);

    println!("{}", result);
}