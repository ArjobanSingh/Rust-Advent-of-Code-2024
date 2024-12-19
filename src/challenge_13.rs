use regex::Regex;
use std::{
    cmp,
    fs::File,
    io::{self, BufRead},
};

const COST_A: i64 = 3;
const COST_B: i64 = 1;
const NUM_TO_ADD: i64 = 10_000_000_000_000;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Game {
    btn_a: Coord,
    btn_b: Coord,
    result: Coord,
}

fn get_min_comb(game: &Game) -> Option<i64> {
    let mut ans: Option<i64> = None;
    let Game {
        btn_a,
        btn_b,
        result,
    } = game;

    let ax = btn_a.x as f64;
    let ay = btn_a.y as f64;
    let bx = btn_b.x as f64;
    let by = btn_b.y as f64;
    let px = result.x as f64;
    let py = result.y as f64;

    // Used Linear Algebra to find count of A and count of B pressess, on coniditon that two lines are not parallel
    let ca = (px * by - py * bx) / (ax * by - ay * bx);
    let cb = (px - ax * ca) / bx;
    println!("ca and cb {ca} {cb}");

    if ca.fract() == 0.0 && cb.fract() == 0.0 {
        ans = Some((ca as i64 * COST_A) + (cb as i64 * COST_B));
    }
    ans
}

pub fn min_tokens(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let re = Regex::new(r"\d+").unwrap();
        let mut result = 0;
        let mut games: Vec<Game> = Vec::new();

        let lines = io::BufReader::new(file).lines();

        let mut current_game_set: Vec<Coord> = Vec::new();

        for line in lines.flatten() {
            if line.trim().is_empty() {
                continue;
            }

            let nums: Vec<i64> = re
                .captures_iter(&line)
                .filter_map(|cap| cap.get(0)?.as_str().parse().ok()) // Only parse valid numbers
                .collect();

            current_game_set.push(Coord {
                x: if current_game_set.len() == 2 {
                    nums[0] + NUM_TO_ADD
                } else {
                    nums[0]
                },
                y: if current_game_set.len() == 2 {
                    nums[1] + NUM_TO_ADD
                } else {
                    nums[1]
                },
            });

            if current_game_set.len() == 3 {
                games.push(Game {
                    btn_a: current_game_set[0],
                    btn_b: current_game_set[1],
                    result: current_game_set[2],
                });
                current_game_set.clear();
            }
        }

        for game in games.iter() {
            if let Some(ans) = get_min_comb(game) {
                result += ans;
            }
        }
        println!("The anser for the Challenge 13 puz 1: {:?}", result);
    }
}
