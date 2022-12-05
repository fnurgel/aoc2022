use std::fs;
use itertools::Itertools;
use enum_map::{Enum};
use std::collections::HashMap;

#[derive(Debug, Enum)]
#[derive(PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors
}

fn play_score(play: &Play) -> u8 {
    return match play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3
    }
}

fn play_beats(play: &Play) -> Play {
    return match play {
        Play::Rock => Play::Scissors,
        Play::Paper => Play::Rock,
        Play::Scissors => Play::Paper
    }
}

fn main() {
    let char_to_play = HashMap::from([
        ("A", Play::Rock),
        ("B", Play::Paper),
        ("C", Play::Scissors),
        ("X", Play::Rock),
        ("Y", Play::Paper),
        ("Z", Play::Scissors),
    ]);
    let contents = fs::read_to_string("input/day2")
        .expect("Couldnt read day file");

    let plays = contents.split("\n")
        .map(|r| r
            .split(" ")
            .collect_vec()
        )
        // .inspect(|s| println!("s: {:?}", s))
        .filter(|v| v.len() == 2)
        .map(|s| (char_to_play.get(s[0]), char_to_play.get(s[1])))
        .filter_map(|(a, b) | match (a, b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None
        })
        .collect::<Vec<_>>();

    let score = plays
        .iter()
        .fold(0, |acc, &(a, b)| {
            let mut round_score: u32 = play_score(b).into();
            if play_beats(b) == *a {
                round_score += 6;
            } else if *b == *a {
                round_score += 3;
            }
            round_score + acc
        });
    print!("{:?}", score);
}
