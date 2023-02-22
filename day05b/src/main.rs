fn main() {
    let mut line_break = false;
    let mut stacks_draw = vec![];
    let mut moves = vec![];
    for line in include_str!("../input.txt").lines() {
        if line.is_empty() {
            line_break = true;
            continue;
        }
        if !line_break {
            stacks_draw.push(line);
            continue;
        }
        let value: [usize; 3] = line
            .split(" ")
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        moves.push(value);
    }

    // could just have hardcoded it as 9 looking at the input
    let row_counter = stacks_draw
        .iter()
        .last()
        .unwrap()
        .split(" ")
        .filter_map(|x| x.parse::<u8>().ok())
        .last()
        .unwrap();

    let mut stacks = (0..row_counter)
        .map(|_| Vec::<char>::new())
        .collect::<Vec<_>>();

    for line in stacks_draw.into_iter().rev() {
        for (idx, x) in line
            .chars()
            .enumerate()
            .filter(|(_, x)| x.is_ascii_alphabetic())
        {
            stacks[idx / 4].push(x);
        }
    }

    for [a, b, c] in moves {
        let origin = &mut stacks[b - 1];
        let moved = origin
            .splice((origin.len() - a)..origin.len(), std::iter::empty())
            .collect::<Vec<_>>();

        stacks[c - 1].extend(moved);
    }

    println!(
        "{}",
        stacks
            .into_iter()
            .map(|mut x| x.pop().unwrap().to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}
