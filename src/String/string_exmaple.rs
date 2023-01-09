#![allow(unused)]
fn main() {
    //1
    let mut s = String::new();
    println!("{s}");
    //2
    let s = String::from("Hello rust");
    println!("{s}");
    //3
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");
    let s = "initial contents".to_string();
    println!("{s}");
    //4 push
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    //5 add
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 여기서 이동되어 더이상 쓸 수 없음을 유의하세요
                       // println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    //6
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{s}");
    //7
    let hello = String::from("Hello, world!");
    let s = &hello[0..4];
    println!("{s}");
    //8 for
    for c in hello.chars() {
        print!("{c}");
    }
    println!();
    for b in hello.bytes() {
        print!("{} ", b);
    }
    println!();
    //9
}
