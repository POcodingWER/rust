// dbg! 


use std::cell::Cell;

fn main () {
    let c = Cell::new(6);

    println!("{c:?}");
    c.set(10);
    println!("{c:?}");
}