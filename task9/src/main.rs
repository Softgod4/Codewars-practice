fn high(input: &str) -> &str {
    let mut alphabet: Vec<&str> = Vec::new();
    let mut score: Vec<u8> = Vec::new();
    let mut input_vector: Vec<&str> = input.split_whitespace().collect();
    for c in " abcdefghijklmnopqrstuvwxyz".split_whitespace() {
        alphabet.push(c);
    }
    
    let mut temp_strings = Vec::new(); 
    
    for c in input.chars() {
        let c_str = c.to_string();
        temp_strings.push(c_str); 
        input_vector.push(&temp_strings[temp_strings.len() - 1]); 
    }

    for i in 0..alphabet.len() {
        let index = alphabet.iter().position(|&r| r == input_vector[i]).unwrap();
        score.push(index as u8);
    }
    
    for i in 0..input_vector.len() {
        println!("{}", input_vector[i]);
    }
    
    return "gaf";
}

fn main() {
    println!("{}", high("man i need a taxi up to ubud"));
}

