use std::{collections::HashMap, fs};
// 1666427
pub fn get_distance_bw_lists(file_path: &str) {
    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2_map: HashMap<u32, u32> = HashMap::new();

    if let Ok(data) = fs::read_to_string(file_path) {
        for (index, value) in data.split_whitespace().enumerate() {
            // In our case we'll only have two lists split over whitespace.
            if index % 2 == 0 {
                // We can safely unwrap knowing it will be converted to i32
                list_1.push(value.parse().unwrap());
            } else {
                list_2_map
                    .entry(value.parse().unwrap())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            };
        }
    }

    let mut distance = 0;
    list_1.iter().for_each(|num| {
        if let Some(&count) = list_2_map.get(num) {
            distance += num * count;
        }
    });
    println!("The solution for 1st challenge is: {distance}");
}
