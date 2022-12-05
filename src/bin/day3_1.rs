use std::fs;
use array_tool::vec::Intersect;

fn main() {
    let contents = fs::read_to_string("input/day3")
        .expect("Couldnt read day file");

    let result = contents.split("\n")
        .inspect(|s| println!("s: {}", s))
        .filter(| s | (*s).len() > 0)
        .map(|s| {
            let mid = s.len() / 2;
            s.split_at(mid)
        })
        .map(|(a, b) | a.as_bytes().to_vec().intersect(b.as_bytes().to_vec()))
        .flat_map(| b | b.iter().map(| bb | *bb as char).collect::<Vec<_>>())
        .map(| c | {
            if c >= 'a' && c <= 'z' {
                (c as u8 - 'a' as u8 + 1) as u32
            } else {
                (c as u8 - 'A' as u8 + 27) as u32
            }
        })
        .collect::<Vec<_>>();
    println!("{:?}", result.iter().sum::<u32>());
}
