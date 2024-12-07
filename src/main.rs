use challenge_1::get_distance_bw_lists;
use challenge_2::get_safe_reports_count;
use challenge_3::get_uncorrupted_mul_ans;
// use challenge_4::search_for_xmas;
use challenge_5::correct_page_order_sum as correct_page_order_sum_puz_1;
use challenge_5_puz_2::correct_page_order_sum;
use challenge_6::get_guard_distinct_pos_size;
use challenge_7_puz_1::bridge_repair_puz_1;
// use challenge_6_puz_1::get_guard_distinct_pos_size;

mod challenge_1;
mod challenge_2;
mod challenge_3;
mod challenge_4;
mod challenge_5;
mod challenge_5_puz_2;
mod challenge_6;
mod challenge_6_puz_1;
mod challenge_7_puz_1;

fn main() {
    // get_distance_bw_lists("src/inputs/challenge_1.txt");
    // get_safe_reports_count("src/inputs/challenge_2.txt");
    // get_uncorrupted_mul_ans("src/inputs/challenge_3.txt");
    // search_for_xmas("src/inputs/challenge_4.txt");
    // correct_page_order_sum_puz_1("src/inputs/challenge_5.txt");
    // correct_page_order_sum("src/inputs/challenge_5.txt");
    // get_guard_distinct_pos_size("src/inputs/challenge_6.txt");
    bridge_repair_puz_1("src/inputs/challenge_7.txt");
}
