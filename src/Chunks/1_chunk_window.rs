
//.chucks //1,2,3  4,5,6  7,8,9
//.windows  //1,2,3

fn main() {
    let num_vec = vec![1,2,3,4,5,6,7,8,9,0];

    for chunk in num_vec.chunks(3){
        println!("chunk is : {chunk:?}")
    }
    for window in num_vec.windows(3){
        println!("widow is {window:?}")
    }

}
