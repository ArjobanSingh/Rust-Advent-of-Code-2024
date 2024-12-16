use regex::Regex;
use std::{
    cmp,
    fs::File,
    io::{self, BufRead},
};

const COST_A: i32 = 3;
const COST_B: i32 = 1;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Game {
    btn_a: Coord,
    btn_b: Coord,
    result: Coord,
}

fn get_min_comb(game: &Game) -> Option<i32> {
    let mut ans: Option<i32> = None;
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
            let tokens_count = (a_press_count * COST_A) + (b_press_count * COST_B);
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

            let nums: Vec<i32> = re
                .captures_iter(&line)
                .filter_map(|cap| cap.get(0)?.as_str().parse().ok()) // Only parse valid numbers
                .collect();

            current_game_set.push(Coord {
                x: nums[0],
                y: nums[1],
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
