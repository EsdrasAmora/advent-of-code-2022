use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("../input.txt");
    let mut sums = vec![];
    let mut current_sum = 0;
    for str in input.lines() {
        if let Ok(value) = str.parse::<u32>() {
            current_sum += value;
        } else {
            sums.push(current_sum);
            current_sum = 0;
        }
    }
    sums.push(current_sum);
    let mut max_sums = BinaryHeap::from(sums);
    let result = (0..3).map(|_| max_sums.pop().unwrap()).sum::<u32>();
    println!("{}", result);
}
