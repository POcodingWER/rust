fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}",v);
    
    let v = vec![1,2,3,4,5];
    println!("{:?}",v);

    let third: &i32 = &v[2];
    println!("{:?}",third);
    let third: Option<&i32> = v.get(2);
    println!("{:?}",third);
    

    let v = vec![1, 2, 3, 4, 5];
    //panicked  없는 참조 불러서 패닉
    // let does_not_exist = &v[100];
    // println!("{:?}",does_not_exist);
    let does_not_exist = v.get(100);
    println!("{:?}",does_not_exist);


}