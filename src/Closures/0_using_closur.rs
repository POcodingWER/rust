fn main() {
    let mut number_one = 8;
    let number_two = 10;

    fn add(a:&mut i32,b:i32) {
        *a += b
    };

    //펑션은 여러번써도됨
    // add(&mut number_one, number_two);
    // println!("{}",number_one);
    // add(&mut number_one, number_two);
    // println!("{}",number_one);
    // add(&mut number_one, number_two);
    // println!("{}",number_one);
     
    let my_closure = |a:&mut i32,b:i32| {
        *a += b
    };

    //이렇게쓰면 안됨 클로저는 한번만써야됨 error는 안나지만 이렇게쓰면 원칙에 어긋남 anti pattern
    my_closure(&mut number_one, number_two);
    println!("{}",number_one);
    my_closure(&mut number_one, number_two);
    println!("{}",number_one);
    // my_closure();
    // my_closure();
}
