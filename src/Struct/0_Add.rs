use std::ops::Add;

#[derive(Debug)]
struct Country {
    name: String,
    population: u32,
    gdp: u32,
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
            gdp,
        }
    }
}
impl Add for Country {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            name:format!("{} and {}",self.name, other.name),
            population: self.population+ other.population,
            gdp:self.gdp+ other.gdp,
        }
    }
}
fn main() {
    let naurn = Country::new("Naurn",10670,160_000_000);
    let vanuatu = Country::new("vanuatu",307815,820_000_000);
    let micronesia = Country::new("micronesia",104468,367_000_000);

    println!("naurn + vanuatu {:?}",naurn + vanuatu + micronesia)
}
