use std::vec;
use std::cmp::{max,min};

fn main() {
    let my_vec = vec![-878,9878979,0,-123,123,-1987237,55];

    let biggest = my_vec
    .into_iter()
    .fold(i32::MIN, |num1,num2| min(num1,num2));

    println!("{biggest:?}")

}
