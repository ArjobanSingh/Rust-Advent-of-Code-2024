use std::{
    fs::File,
    io::{self, BufRead},
};

fn rearrange_none_at_end(vec: &mut Vec<Option<u32>>) {
    let mut start = 0;
    let mut end = vec.len() - 1;
    while start < end {
        // iterate while the start is at Some(u32) value.
        while start < end && vec[start].is_some() {
            start += 1;
        }

        while start < end && vec[end].is_none() {
            end -= 1;
        }

        vec.swap(start, end);
        start += 1;
        end -= 1;
    }
}

pub fn disk_fragmenter_puz_1(file_path: &str) {
    let mut vec: Vec<Option<u32>> = Vec::new();
    if let Ok(file) = File::open(file_path) {
        let mut reader = io::BufReader::new(file);

        let mut line = String::new();
        if let Ok(size) = reader.read_line(&mut line) {
            if size > 0 {
                let mut id = 0;
                for (ch_idx, ch) in line.trim().chars().enumerate() {
                    if let Some(num) = ch.to_digit(10) {
                        if ch_idx % 2 == 0 {
                            // Add id that many times
                            for _ in 0..num {
                                vec.push(Some(id));
                            }
                            id += 1;
                        } else {
                            // add empty space in vec
                            for _ in 0..num {
                                vec.push(None);
                            }
                        }
                    }
                }
            }
        }
    }

    rearrange_none_at_end(&mut vec);
    let ans = vec
        .into_iter()
        .filter_map(|x| x)
        .enumerate()
        .fold(0, |sum, (idx, id)| sum + (id as usize * idx));
    println!("Here's the vec {:?}", ans);
}
