fn main() {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    //filter => 변환갑이boll type
    //filter_map => 변환된값이 리턴
    let filtered_months = months
        .into_iter() // make an iter
        .filter(|month| month.len() < 5) // We don't want months more than 5 bytes in length.
        // We know that each letter is one byte so .len() is fine
        .filter(|month| month.contains("u")) // Also we only like months with the letter u
        .collect::<Vec<&str>>();

    println!("{:?}", filtered_months);
}
