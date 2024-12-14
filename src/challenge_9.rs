use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Clone, Copy)]
struct Item {
    id: i32,
    position: isize,
    size: isize,
    is_swapped: bool,
}

fn rearrange_none_at_end(vec: &mut Vec<Option<Item>>) {
    let mut i = (vec.len() - 1) as isize;
    // Start from last and go till first and every time check if certain whole file
    // size can be put at smaller index and move. Else do not move that whole file
    while i >= 0 {
        if let Some(item) = vec[i as usize] {
            // If we already swapped this, no need to check it further
            // as we won't have any bigger size for it ahead
            if item.is_swapped {
                i -= item.size;
                continue;
            }

            // file start idx
            let item_start = item.position - item.size + 1;
            if item_start < 0 {
                break;
            }
            let mut found_idx_option: Option<isize> = None;
            let mut empty_space_size = 0;

            // from start to till this place, find the empty block which can hold whole file of item.size
            for j in 0..item_start {
                if let None = vec[j as usize] {
                    empty_space_size += 1;
                    if empty_space_size == item.size {
                        found_idx_option = Some((j - empty_space_size + 1) as isize);
                        break;
                    }
                } else {
                    empty_space_size = 0;
                }
            }

            // Swap file chunks with that empty space
            if let Some(found_idx) = found_idx_option {
                for j in 0..empty_space_size {
                    vec.swap((found_idx + j) as usize, i as usize);
                    if let Some(Some(item)) = vec.get_mut((found_idx + j) as usize) {
                        item.is_swapped = true;
                    }
                    i -= 1;
                }
            } else {
                i -= item.size;
            }
        } else {
            // Empty space, just decrement
            i -= 1;
        }
    }
}

pub fn disk_fragmenter(file_path: &str) {
    let mut vec: Vec<Option<Item>> = Vec::new();
    if let Ok(file) = File::open(file_path) {
        let mut reader = io::BufReader::new(file);

        let mut line = String::new();
        if let Ok(size) = reader.read_line(&mut line) {
            if size > 0 {
                let mut id = 0;
                for (ch_idx, ch) in line.trim().chars().enumerate() {
                    if let Some(num) = ch.to_digit(10) {
                        let file_size = num as isize;
                        if ch_idx % 2 == 0 {
                            let size_till = vec.len() as isize;
                            // Add id that many times
                            for _ in 0..file_size {
                                vec.push(Some(Item {
                                    id,
                                    size: file_size as isize,
                                    position: size_till - 1 + file_size,
                                    is_swapped: false,
                                }));
                            }
                            id += 1;
                        } else {
                            // add empty space in vec
                            for _ in 0..file_size {
                                vec.push(None);
                            }
                        }
                    }
                }
            }
        }
    }

    rearrange_none_at_end(&mut vec);
    let ans = vec.into_iter().enumerate().fold(0, |sum, (idx, item)| {
        if let Some(item) = item {
            return sum + (item.id as usize * idx);
        }
        sum
    });
    println!("Here's the 9th ans {:?}", ans);
}
