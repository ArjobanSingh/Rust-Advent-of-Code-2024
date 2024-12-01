use std::fs;

pub fn get_distance_bw_lists(file_path: &str) {
    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();

    if let Ok(data) = fs::read_to_string(file_path) {
        for (index, value) in data.split_whitespace().enumerate() {
            // In our case we'll only have two lists split over whitespace.
            // and we safely unwrap knowing it will be converted to i32
            if index % 2 == 0 {
                list_1.push(value.parse().unwrap());
            } else {
                list_2.push(value.parse().unwrap());
            };
        }
    }

    list_1.sort_unstable();
    list_2.sort_unstable();

    let mut distance = 0;
    list_1.iter().enumerate().for_each(|(idx, &value)| {
        if let Some(&other) = list_2.get(idx) {
            distance += value.abs_diff(other);
        }
    });
    println!("The solution for 1st challenge is: {distance}");
}
