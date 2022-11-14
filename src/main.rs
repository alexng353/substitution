fn main() {
    // args
    let args = std::env::args().collect::<Vec<String>>();

    let mode: &str = &args[1].to_lowercase();

    let substitution = args[2].clone();

    // if length subs not equals 26, throw error
    if substitution.len() != 26 {
        panic!("Substitution must be 26 characters long");
    }

    let allcaps = substitution.to_uppercase();

    let alllower = substitution.to_lowercase();

    let word = args[3].clone();

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let alphalower = "abcdefghijklmnopqrstuvwxyz";

    let mut newword = String::new();

    // match mode = e, enc, encode

    match mode {
        "e" | "enc" | "encode" => {
            for c in word.chars() {
                if alphabet.contains(c) {
                    let index = alphabet.find(c).unwrap();
                    newword.push(allcaps.chars().nth(index).unwrap());
                } else if alphalower.contains(c) {
                    let index = alphalower.find(c).unwrap();
                    newword.push(alllower.chars().nth(index).unwrap());
                } else {
                    newword.push(c);
                }
            }
        }
        "d" | "dec" | "decode" => {
            for c in word.chars() {
                if allcaps.contains(c) {
                    let index = allcaps.find(c).unwrap();
                    newword.push(alphabet.chars().nth(index).unwrap());
                } else if alllower.contains(c) {
                    let index = alllower.find(c).unwrap();
                    newword.push(alphalower.chars().nth(index).unwrap());
                } else {
                    newword.push(c);
                }
            }
        }
        &_ => {
            for c in word.chars() {
                if allcaps.contains(c) {
                    let index = allcaps.find(c).unwrap();
                    newword.push(alphabet.chars().nth(index).unwrap());
                } else if alllower.contains(c) {
                    let index = alllower.find(c).unwrap();
                    newword.push(alphalower.chars().nth(index).unwrap());
                } else {
                    newword.push(c);
                }
            }
        }
    }

    println!("{}", newword);
}
