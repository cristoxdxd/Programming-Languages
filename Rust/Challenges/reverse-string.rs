use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a string to reverse:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let mut output = String::new();
    for c in input.chars().rev() {
        output.push(c);
    }
    println!("{}", output);
}