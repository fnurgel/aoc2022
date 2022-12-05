use std::fs;
use array_tool::vec::Intersect;

fn main() {
    let contents = fs::read_to_string("input/day3")
        .expect("Couldnt read day file");

    let backpacks = contents.split("\n")
        .inspect(|s| println!("s: {}", s))
        .filter(|s| (*s).len() > 0)
        .collect::<Vec<_>>();

    let tags = backpacks.chunks(3)
        .inspect(|s| println!("s: {:?}", s))
        .map(|a| match a {
            [a, b, c] =>
                a.as_bytes().to_vec()
                    .intersect(b.as_bytes().to_vec())
                    .intersect(c.as_bytes().to_vec()),
            _ => vec![]
        })
        .flat_map(|b| b.iter().map(|bb| *bb as char).collect::<Vec<_>>())
        .map(|c| {
            if c >= 'a' && c <= 'z' {
                (c as u8 - 'a' as u8 + 1) as u32
            } else {
                (c as u8 - 'A' as u8 + 27) as u32
            }
        })
        .collect::<Vec<_>>();
    println!("{:?}", tags.iter().sum::<u32>());
}
