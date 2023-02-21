use std::collections::HashSet;
use std::hash::Hash;
use std::result;

fn intersections<T>(sets: &[&HashSet<T>]) -> HashSet<T>
where
    T: Clone + Eq + Hash,
{
    match sets.len() {
        0 => HashSet::new(),
        _ => sets[1..].iter().fold(sets[0].clone(), |mut acc, set| {
            acc.retain(|item| set.contains(item));
            acc
        }),
    }
}

fn main() {
    let v = include_str!("../input.txt")
        .lines()
        .map(|x| x.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    let mut result: u32 = 0;

    for i in (0..v.len()).step_by(3) {
        let [a, b, c] = [&v[i], &v[i + 1], &v[i + 2]];
        let xxx = [a, b].iter().fold(c.clone(), |mut acc, set| {
            acc.retain(|item| set.contains(item));
            acc
        });
        let duplicated = xxx.iter().next().unwrap().clone();
        result += match duplicated {
            'a'..='z' => duplicated as u8 - 'a' as u8 + 1,
            'A'..='Z' => duplicated as u8 - 'A' as u8 + 27,
            _ => panic!("Invalid char, current: {}", duplicated),
        } as u32;
    }

    println!("{:?}", result);
}
