#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
struct Point2<T,U> {
    x: T,
    y: U,
}

fn main() {
    //1
    let integer = Point { x: 5, y: 10 };
    println!("{:?}",integer);
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}",float);
    
    // 2 error 타입이달라서 에러
    // let wont_work = Point { x: 5, y: 4.0 };
    // println!("{:?}",wont_work);
    
    //3 error 해결
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("{:?} {:?} {:?}",both_integer,both_float,integer_and_float);
}
