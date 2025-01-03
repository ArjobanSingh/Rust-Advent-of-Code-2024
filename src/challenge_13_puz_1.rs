use regex::Regex;
use std::{
    cmp,
    fs::File,
    io::{self, BufRead},
};

const COST_A: u8 = 3;
const COST_B: u8 = 1;
const NUM_TO_ADD: u64 = 10_000_000_000_000;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Copy)]
struct Game {
    btn_a: Coord,
    btn_b: Coord,
    result: Coord,
}

fn get_min_comb(game: &Game) -> Option<u64> {
    let mut ans: Option<u64> = None;
    let min_x = cmp::min(game.btn_a.x, game.btn_b.x);
    let min_y = cmp::min(game.btn_a.y, game.btn_b.y);

    for count in 0.. {
        let a_press_count = count;
        let mut curr = Coord {
            x: game.btn_a.x * a_press_count,
            y: game.btn_a.y * a_press_count,
        };
        let mut b_press_count = 0;

        while curr.x < game.result.x && curr.y < game.result.y {
            curr.x += game.btn_b.x;
            curr.y += game.btn_b.y;
            b_press_count += 1;
        }

        if curr.x == game.result.x && curr.y == game.result.y {
            let tokens_count =
                (a_press_count * (COST_A as u64)) + (b_press_count * (COST_B as u64));
            if let Some(prev_ans) = ans {
                ans = Some(cmp::min(prev_ans, tokens_count));
            } else {
                ans = Some(tokens_count);
            }
        }

        // kind of hacky way to keep iterating till the smalles x and y deltas go across the result
        // TODO: improve this logic
        if min_x * count > game.result.x || min_y * count > game.result.y {
            break;
        }
    }
    ans
}

pub fn min_tokens_1(file_path: &str) {
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

            let nums: Vec<u64> = re
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
