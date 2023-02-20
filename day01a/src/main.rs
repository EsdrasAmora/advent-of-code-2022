fn main() {
    let input = include_str!("../input.txt");
    let mut result = 0;
    let mut current_sum = 0;
    for str in input.lines() {
        if let Ok(value) = str.parse::<u32>() {
            current_sum += value;
        } else {
            result = std::cmp::max(result, current_sum);
            current_sum = 0;
        }
    }
    println!("{}", std::cmp::max(result, current_sum))
}
