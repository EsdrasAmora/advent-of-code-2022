use std::collections::HashMap;

fn main() {
    let input = &mut include_str!("../input.txt").lines().peekable();
    let mut file_tree = HashMap::new();
    let mut current_path = String::with_capacity(100);
    current_path.push('/');
    let mut result = 0;

    while let Some(it) = input.next() {
        let mut size = 0;
        match it {
            "$ cd .." => {
                // println!("before:{}", current_path);
                current_path.truncate(
                    current_path
                        .rfind("/")
                        .expect(&format!("did not found a `/`, current:{}", current_path))
                        + 1,
                ); // remove the last `/path`
                   // println!("after:{}", current_path)
            }
            "$ ls" => {
                size = std::iter::from_fn(|| input.next_if(|i| i.as_bytes()[0] != b'$')) // equivalent to starts_with("$")
                    .filter(|i| i.as_bytes()[0] != b'd') // ignore dirs
                    .filter_map(|i| {
                        i.split(" ")
                            .next()
                            .expect("string did not have any spaces?")
                            .parse::<u32>()
                            .ok()
                    })
                    .sum();
                println!("size: {size}");
                // .for_each(|i| println!("value: {i}"))
            }
            _ if it.starts_with("$ cd") => {
                let dir_name = it
                    .split(" ")
                    .last()
                    .expect(&format!("did not found a path on {}", it))
                    .to_owned();

                if dir_name == "/" {
                    continue;
                }

                current_path.push_str(&dir_name);

                // could not find a way to not clone here, is it really impossible?
                file_tree.entry(current_path.clone()).or_insert(0);
            }
            _ => {
                unreachable!("All non command lines will be handled within `ls` block")
            }
        };
        // println!("{it} -- {current_path}");
        if size <= 100_000 {
            result += size;
        }
    }

    println!("\n\nresult:{result}");
}
