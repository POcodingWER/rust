use std::collections::HashMap;

macro_rules! map {
    ($( $key:expr => $value:expr),*) => {{
        let mut hm = HashMap::new();
        $(hm.insert($key, $value);)*
    }};
}
// macro_rules! map (
//     {$($key:expr => $value:expr), + } => {
//         {
//             let mut m = HashMap::new();
//             $(
//                 m.insert($key, $value);
//              )+
//             m
//         }
//     };
// );
fn main() {
    let user = map!(
        "name" => "Finn",
        "gender" => "Boy"
    );
    println!("User {:?}", user);
}
