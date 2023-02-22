fn main() {
    let result = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let val = x
                .split(",")
                .map(|y| {
                    y.split("-")
                        .map(|z| z.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<_>>();
            let (l_min, l_max) = (val[0][0], val[0][1]);
            let (r_min, r_max) = (val[1][0], val[1][1]);
            if l_min <= r_min && l_max >= r_max || r_min <= l_min && r_max >= l_max {
                return 1;
            }
            return 0;
        })
        .sum::<u32>();
    println!("{:?}", result);
}
