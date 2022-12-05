use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day1")
        .expect("Couldnt read day1 file");

    let cal = contents.split("\n")
        .map(|s| s.trim().parse::<i128>())
        .collect::<Vec<_>>();

    let mut result = cal.iter().fold(vec![0], |mut acc, element | {
        match element {
            Ok(v) => {
                let last_pos = acc.len() - 1;
                acc[last_pos] += v;
            },
            _err => acc.push(0)
        }
        acc
    });
    result.sort();
    let three_highest = &result[result.len() - 3..];
    println!("{:?}", three_highest.iter().fold(0, |acc, e| acc + *e));
}
