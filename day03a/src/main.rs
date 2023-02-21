use std::collections::HashSet;

fn main() {
    let result = include_str!("../input.txt")
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .map(|x| {
            let (left, right) = x.split_at(x.len() / 2);
            let left_char_set: HashSet<&char> = left.iter().collect();
            let duplicated = right
                .iter()
                .find(|x| left_char_set.contains(x))
                .unwrap()
                .to_owned();
            let result = match duplicated {
                'a'..='z' => duplicated as u8 - 'a' as u8 + 1,
                'A'..='Z' => duplicated as u8 - 'A' as u8 + 27,
                _ => panic!("Invalid char, current: {}", duplicated),
            };
            result as u32
        })
        .sum::<u32>();

    println!("{}", result);
}
