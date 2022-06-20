fn search_min_pair(nums1: Vec<i32>, nums2: Vec<i32>, threshold: i32) -> i32 {
    let mut min_val: i32 = 20000000;
    for num1 in &nums1 {
        for num2 in &nums2 {
            let pair: i32 = *num1 + *num2;
            if pair < threshold {
                continue;
            }
            if pair < min_val {
                min_val = pair;
            }
        }
    }
    min_val
}

fn main() {
    let nums1: Vec<i32> = vec![8, 5, 4];
    let nums2: Vec<i32> = vec![4, 1, 9];
    let threshold: i32 = 10;
    let result: i32 = search_min_pair(nums1, nums2, threshold);

    println!("{}", result);
}