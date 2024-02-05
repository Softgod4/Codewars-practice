fn are_you_playing_banjo(name: &str) -> String {
    if name.contains(&"r") || name.contains(&"R") {
        return name.to_owned() + " plays banjo"
    } else { return name.to_owned() + " does not play banjo" }
}

fn main() {
    println!("{}", are_you_playing_banjo("soman"));
}
