use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../input.txt").chars().collect::<Vec<_>>();
    // took me some time to discover that i needed to `rev` this iterator before collecting into the deque;
    let mut current_window: VecDeque<&char> = input.iter().take(14).rev().collect();
    let mut char_counter = input
        .iter()
        .take(14)
        .fold(HashMap::with_capacity(14), |mut acc, it| {
            *acc.entry(it).or_insert(0) += 1;
            acc
        });

    for (idx, char) in input.iter().enumerate().skip(14) {
        if char_counter.len() == 14 {
            return println!("result: {}", idx);
        }
        let key_to_decrement = current_window.pop_back().unwrap();
        let value = char_counter.get_mut(key_to_decrement).unwrap();
        *value -= 1;
        if *value == 0 {
            char_counter.remove(key_to_decrement);
        }
        *char_counter.entry(char).or_insert(0) += 1;
        current_window.push_front(char);
    }
    println!("something went wrong");
}
