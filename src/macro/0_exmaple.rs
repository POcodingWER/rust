macro_rules! create_function {
    //ident 함수명같은 밖으로뺄 값
    ($func_name:ident) => {
        fn $func_name(a:i32,b:i32) {
            println!("You called {:?}()",
                     a+b);
        }
    };
}

macro_rules! print_result {
    //expr 함수안으로 들어갈 값
    ($expression:expr) => {
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

fn main() {
    create_function!(foo);
    create_function!(bar);
    foo(1,2);
    bar(2,3);

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
