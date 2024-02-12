use std::io;

fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase(); // Convert input string to lowercase
    let reversed: String = input.chars().rev().collect(); // Reverse the input string
    input == reversed // Check if the original and reversed strings are equal
}

fn main() {
    println!("Enter a string:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Trim whitespace and newline characters
    
    if is_palindrome(input) {
        println!("\"{}\" is a palindrome.", input);
    } else {
        println!("\"{}\" is not a palindrome.", input);
    }
}
