fn sum_string(num_str: &str) -> i32 {
    let mut result: i32 = 0;
    let between: usize = num_str.len() - 1;
    for bit in 0..(1<<between) {
        let mut temp: i32 = 0;
        for (idx, c) in num_str.chars().enumerate() {
            if idx == between { 
                break;
            }
            temp += c as i32 - 48;  // 48はASCIIコードで'0'
            if bit & (1<<idx) > 0 {
                result += temp;
                temp = 0;
            } else {
                temp *= 10;
            }
        }
        let end_num: i32 = num_str.chars().nth(between).unwrap() as i32 - 48;  // 48はASCIIコードで'0'
        result += temp + end_num;
    }
    result
}


fn main() {
    let num_str: &str = "125";
    let result: i32 = sum_string(num_str);

    println!("{}", result);
}