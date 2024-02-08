fn high(input: &str) -> &str {
    let mut alphabet: Vec<String> = Vec::new();
    let mut score: Vec<u8> = Vec::new();
    let mut input_vector = input.collect::<Vec<&str>>();
    for c in " abcdefghijklmnopqrstuvwxyz".chars() {
        alphabet.push(c.to_string());
    }
    for i in 0..alphabet.len() {
        let index = alphabet.iter().position(|r| r.as_str() == input_vector[i]).unwrap();
        score.push(index.try_into().unwrap());
    }
    for i in input_vector {
        println!("{}", i);
    }
    return "gaf";
} 

fn main() {
    println!("{}", high("man i need a taxi up to ubud"));
}
