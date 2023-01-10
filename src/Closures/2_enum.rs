fn main() {
    let num_vec = vec![2,4,6];

    let a:Vec<(_,_)> =num_vec
        .iter()
        .enumerate()
        .map(|(index, number)|{println!("{}  {}",index , number);
        return (index,number*2);
    })
        .collect();

    println!("{:?}",a);
}
