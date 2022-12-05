use std::fs;
use itertools::Itertools;
use enum_map::{Enum};
use std::collections::HashMap;

#[derive(Debug, Enum)]
#[derive(PartialEq, Eq, Clone)]
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

fn play_looses(play: &Play) -> Play {
    return match play {
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
        Play::Scissors => Play::Rock,
    }
}

fn play_draw(play: &Play) -> Play {
    return match play {
        Play::Rock => Play::Rock,
        Play::Paper => Play::Paper,
        Play::Scissors => Play::Scissors,
    }
}

type PlayResolver = fn(&Play) -> Play;

fn main() {
    let char_to_play = HashMap::from([
        ("A", Play::Rock),
        ("B", Play::Paper),
        ("C", Play::Scissors),
    ]);
    let resolve_play = HashMap::from([
        ("X", play_beats as PlayResolver),
        ("Y", | p | (*p).clone()),
        ("Z", play_looses)
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
        .map(|s| (char_to_play.get(s[0]), resolve_play.get(s[1])))
        .filter_map(|(a, b) | match (a, b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None
        })
        .map(|(a, b) | (a, b(a)))
        .collect::<Vec<_>>();

    let score = plays
        .iter()
        .fold(0, |acc, (a, b)| {
            let mut round_score: u32 = play_score(&b).into();
            if play_beats(&b) == **a {
                round_score += 6;
            } else if b == *a {
                round_score += 3;
            }
            round_score + acc
        });
    print!("{:?}", score);
}
