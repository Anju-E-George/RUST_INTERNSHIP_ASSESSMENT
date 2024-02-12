use std::io;

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if let Some(shortest) = shortest_word(&input) {
        println!("The shortest word in the string is: {}", shortest);
    } else {
        println!("No words found in the input string");
    }
}
