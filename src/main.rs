use challenge_1::get_distance_bw_lists;
use challenge_2::get_safe_reports_count;
use challenge_3::get_uncorrupted_mul_ans;

mod challenge_1;
mod challenge_2;
mod challenge_3;

fn main() {
    // get_distance_bw_lists("src/inputs/challenge_1.txt");
    // get_safe_reports_count("src/inputs/challenge_2.txt");
    get_uncorrupted_mul_ans("src/inputs/challenge_3.txt");
}
