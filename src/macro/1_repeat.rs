// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // 1개면 바로 리턴:
    ($x:expr) => ($x);
    // 1개이상이면 find_min 계속 repeat 
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
