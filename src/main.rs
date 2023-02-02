use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut add_num = 0;
        for i in 1..=5 {
            add_num += i;
        }
        println!("{}",add_num); 
        1
    }
}
fn main() {
    let make_point_arry = Solution::count_vowel_strings(2);
    println!("result {:?}", make_point_arry)
}
