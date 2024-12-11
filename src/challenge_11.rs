use std::{
    fs::File,
    io::{self, BufRead},
};

const N: u8 = 25;

// fn remove_leading_zeroes(chunk: &str) -> String {
//     let mut leading_count = 0;
//     for (idx, &byte) in chunk.as_bytes().iter().enumerate() {
//         if idx == chunk.len() - 1 || (byte as char) != '0' {
//             break;
//         }
//         leading_count += 1;
//     }

//     chunk[leading_count..].to_string()
// }

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

pub fn stone_count(file_path: &str) {
    let mut vec: Vec<u64> = Vec::new();
    if let Ok(file) = File::open(file_path) {
        let mut reader = io::BufReader::new(file);

        let mut line = String::new();
        if let Ok(size) = reader.read_line(&mut line) {
            if size <= 0 {
                return;
            }

            for str in line.split_whitespace() {
                if let Ok(num) = str.parse::<u64>() {
                    vec.push(num);
                }
            }
        }

        for _ in 0..N {
            let mut new_vec: Vec<u64> = Vec::new();
            for &num in vec.iter() {
                if num == 0 {
                    new_vec.push(1);
                    continue;
                }

                let (is_even_digits, digits) = get_is_even_digits(num);
                if is_even_digits {
                    let (num_1, num_2) = split_num_at_mid(num, digits);
                    new_vec.push(num_1);
                    new_vec.push(num_2);
                    continue;
                }

                // normal case
                new_vec.push(num * 2024);
            }
            vec = new_vec;
        }
        println!("Here's the vec and {}", vec.len());
    }
}
