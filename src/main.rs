use challenge_1::get_distance_bw_lists;
use challenge_2::{get_safe_reports_count, get_safe_reports_count_ch_1};

mod challenge_1;
mod challenge_2;

fn main() {
    // get_distance_bw_lists("src/inputs/challenge_1.txt");
    get_safe_reports_count_ch_1("src/inputs/challenge_2.txt");
    get_safe_reports_count("src/inputs/challenge_2.txt");
}
