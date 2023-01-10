
fn main() {
    let numbers_together = "84898746415139874651328794651329846513";
    // println!("{:?}",numbers_together.char_indices());

    for (i,num) in numbers_together.char_indices() {
        match (i %3,num) {
            (0..=1, num ) => print!("{}",num),
            _ => print!("{}\t",num),
            
        }
    }
    
}
