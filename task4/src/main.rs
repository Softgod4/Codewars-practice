fn maskify(cc: &str) -> String {
    if cc.len() <= 4 {
        return cc.to_string();
    }
    let mut _charvec: Vec<char> = cc.chars().rev().collect();
    let mut _resultvec: Vec<char> = vec![];
    for n in 0..4 {
        _resultvec.push(_charvec[n])
    }
    let symbol: char = '#';
    for _ in 0.._charvec.len() - 4 {
        _resultvec.push(symbol);
    }
    let _resultvec = _resultvec.iter().rev().collect();
    return _resultvec;
}

fn main() {
    println!("{}", maskify("4556364607935616"));
}
