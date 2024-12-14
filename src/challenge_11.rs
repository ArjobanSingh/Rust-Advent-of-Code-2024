use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

const N: u16 = 75;

fn get_is_even_digits(mut num: u64) -> (bool, u32) {
    let mut digits: u32 = 0;
    while num >= 1 {
        num /= 10;
        digits += 1;
    }
    (digits % 2 == 0, digits)
}

fn split_num_at_mid(num: u64, digits: u32) -> (u64, u64) {
    let mut divident = 1;
    for _ in 0..digits / 2 {
        divident *= 10;
    }

    let num_1 = num / divident;
    let num_2 = num % divident;
    (num_1, num_2)
}

fn transform_nums(num: u64, count: u64, map: &mut HashMap<u64, u64>) {
    if num == 0 {
        *map.entry(1).or_insert(0) += count;
        return;
    }

    let (is_even_digits, digits) = get_is_even_digits(num);
    if is_even_digits {
        let (num_1, num_2) = split_num_at_mid(num, digits);
        *map.entry(num_1).or_insert(0) += count;
        *map.entry(num_2).or_insert(0) += count;
        return;
    }

    // normal case
    *map.entry(num * 2024).or_insert(0) += count;
}

pub fn stone_count(file_path: &str) {
    let mut map: HashMap<u64, u64> = HashMap::new();
    if let Ok(file) = File::open(file_path) {
        let mut reader = io::BufReader::new(file);

        let mut line = String::new();
        if let Ok(size) = reader.read_line(&mut line) {
            if size <= 0 {
                return;
            }

            for str in line.split_whitespace() {
                if let Ok(num) = str.parse::<u64>() {
                    *map.entry(num).or_insert(0) += 1;
                }
            }
        }

        for _ in 0..N {
            let mut new_map: HashMap<u64, u64> = HashMap::new();

            for (num, count) in map {
                transform_nums(num, count, &mut new_map);
            }
            map = new_map;
        }

        println!(
            "Here's the map size {} and ans {}",
            map.len(),
            map.values().fold(0, |acc, val| acc + val)
        );
    }
}
