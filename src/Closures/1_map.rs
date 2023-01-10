fn main() {
    let num_vec = vec![2,4,6];

    //rust map => 변환받을 변수필요  변환값있다는뜻
    //for_each => 변환받을 변수 없음  변환값없다는뜻
    let double_vec: Vec<i32> =num_vec
        .iter()
        .map(|num| num*2)
        .collect();

    println!("{:?}",double_vec);
}
