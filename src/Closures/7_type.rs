fn fn_closure<F>(f: F)
where
    F: Fn(),
{
    f();
}
fn fn_mut_closure<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}
fn fn_once_closure<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
fn main() {
    let mut my_string = String::from("Hello there");
    //읽기만가능
    fn_closure(|| println!("{my_string}"));
    println!("{my_string}");
    //수정가능
    fn_mut_closure(|| {
        my_string.push('a');
        println!("{my_string}")
    });
    println!("{my_string}");

    //값을 넘겨받음
    fn_once_closure(|| {
        my_string.push('b');
        println!("{my_string}");
        drop(my_string);
    });
    //드롭되서 값이 날라감
    // println!("{my_string}");

}
