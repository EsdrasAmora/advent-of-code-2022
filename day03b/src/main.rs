use std::collections::HashSet;

fn main() {
    let result = include_str!("../input.txt")
        .lines()
        .map(|x| x.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .windows(3)
        .step_by(3)
        .map(|it| {
            let [a, b, c] = [&it[0], &it[1], &it[2]];
            let duplicated = [a, b]
                .iter()
                .fold(c.clone(), |mut acc, set| {
                    acc.retain(|item| set.contains(item));
                    acc
                })
                .iter()
                .next()
                .unwrap()
                .clone();
            let result = match duplicated {
                'a'..='z' => duplicated as u8 - 'a' as u8 + 1,
                'A'..='Z' => duplicated as u8 - 'A' as u8 + 27,
                _ => panic!("Invalid char, current: {}", duplicated),
            };
            result as u32
        })
        .sum::<u32>();

    println!("{:?}", result);
}
