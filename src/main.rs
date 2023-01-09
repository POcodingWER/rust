fn main() {
    //1
    let numbers = vec![34, 50, 25, 100, 65];
    let mut largest = numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
    //2
    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
    //3    1,2가 중복
    fn find_largest(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    let result = find_largest(&numbers);
    println!("The largest number is {}", result);
    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = find_largest(&numbers);
    println!("The largest number is {}", result);
}
