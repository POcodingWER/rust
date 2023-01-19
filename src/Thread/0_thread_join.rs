



use std::thread;  

fn main() {
    let mut join_vec =  vec![];

    for i in 0..10  {
        let handle =  thread::spawn(||{
            println!("I am printing something");
        });
        join_vec.push(handle);
    }

    join_vec.into_iter().for_each(|handle| handle.join().unwrap());
     

}