// dbg! 


use std::rc::Rc;


fn  takes_a_string(input:Rc<String>) {
    print!("{input}")
}

fn main () {
    let my_stying = Rc::new("hellow".to_string());
    
    takes_a_string(my_stying);
}