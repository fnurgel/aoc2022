use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day1")
        .expect("Couldnt read day1 file");

    let cal = contents.split("\n")
        .inspect(|s| println!("s: {}", s))
        .map(|s| s.trim().parse::<i128>())
        .collect::<Vec<_>>();

    let result = cal.iter().fold(vec![0], |acc, element | {
        let mut new_acc = acc.clone();
        if element.is_ok() {
            if acc.len() > 0 {
                new_acc[acc.len() - 1] += element.as_ref().unwrap();
            }
        } else {
            new_acc.push(0);
        }
        new_acc
    });
    println!("{:?}", result.iter().max().unwrap());
}
