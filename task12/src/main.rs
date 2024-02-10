fn get_age(age: &str) -> u32 {
    let first: u32 = age.chars().nth(0);
    return first;
}

fn main() {
    println!("{}", get_age("2 years old"));
}
