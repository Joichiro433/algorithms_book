fn get_753_times(target: i32, num: i32, valid_bit: i32, result_patterns: &mut Vec<i32>) {
    if num > target {
        return;
    }
    if valid_bit == 0b111 {
        result_patterns.push(num);
    }

    get_753_times(target, num * 10 + 7, valid_bit | 0b100, result_patterns);
    get_753_times(target, num * 10 + 5, valid_bit | 0b010, result_patterns);
    get_753_times(target, num * 10 + 3, valid_bit | 0b001, result_patterns);

}



fn main() {
    let target: i32 = 575;
    let mut result_patterns: Vec<i32> = vec![];
    get_753_times(target, 0, 0b000, &mut result_patterns);

    println!("{:?}", result_patterns);
    println!("{}", result_patterns.len());
}