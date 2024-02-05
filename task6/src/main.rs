fn create_phone_number(numbers: &[u8]) -> String {
    let mut string_vector: Vec<String> = Vec::new();
    for n in 0..numbers.len() {
        string_vector.push(numbers[n].to_string());
    }
    let number = String::from(
        "(".to_owned() + &string_vector[0]
        + &string_vector[1] + &string_vector[2] 
        + ")" + " " + &string_vector[3]
        + &string_vector[4] + &string_vector[5]
        + "-" + &string_vector[6] + &string_vector[7]
        +&string_vector[8] + &string_vector[9]); 
    return number;
}
fn main() {
    println!("{}", create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
}

#[test]
fn returns_expected() {
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(
        create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        "(111) 111-1111"
    );
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
        "(123) 456-7899"
    );
}
