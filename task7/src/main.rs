fn is_pangram(s: &str) -> bool {
    let mut vector_type: Vec<char> = Vec::new();
    for c in s.chars() {
        if  c.is_alphabetic() && !vector_type.contains(&c) {
            vector_type.push(c);
        }
        return false;
    }   
    return true;
}

fn main() {
    println!("{}", is_pangram("This isn't a pangram!"));
    println!("{}", is_pangram("The quick, brown fox jumps over the lazy dog!"));
}
