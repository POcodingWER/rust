
// RefCell  싱글스레드
// Mutex  멀티 스레드
// Rc   RefCell랑 같이사용
// Arc mutex랑 같이사용

use std::sync::{Mutex,RwLock};
fn main() {
    let my_mutex = Mutex::new(5);

    let mut mutex_changer = my_mutex.lock().unwrap();
    // *mutex_changer = 6;
    // drop(mutex_changer);
    // println!("{my_mutex:?}")
    let mut other_nutex_change = my_mutex.try_lock();

    if let Ok(value) = other_nutex_change {
        println!("The mutexchanger has: {value}");
    }else {
        println!("Didn't get a lock");
    }
    println!("{my_mutex:?}");

}