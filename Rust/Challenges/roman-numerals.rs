// I = 1
// V = 5
// X = 10
// L = 50
// C = 100
// D = 500
// M = 1000

use std::io;

fn roman_numerals(number: u32) -> String {
    let mut result = String::new();
    let mut number = number;
    while number > 0 {
        if number >= 1000 {
            result.push_str("M");
            number -= 1000;
        } else if number >= 900 {
            result.push_str("CM");
            number -= 900;
        } else if number >= 500 {
            result.push_str("D");
            number -= 500;
        } else if number >= 400 {
            result.push_str("CD");
            number -= 400;
        } else if number >= 100 {
            result.push_str("C");
            number -= 100;
        } else if number >= 90 {
            result.push_str("XC");
            number -= 90;
        } else if number >= 50 {
            result.push_str("L");
            number -= 50;
        } else if number >= 40 {
            result.push_str("XL");
            number -= 40;
        } else if number >= 10 {
            result.push_str("X");
            number -= 10;
        } else if number >= 9 {
            result.push_str("IX");
            number -= 9;
        } else if number >= 5 {
            result.push_str("V");
            number -= 5;
        } else if number >= 4 {
            result.push_str("IV");
            number -= 4;
        } else if number >= 1 {
            result.push_str("I");
            number -= 1;
        }
    }
    result
}

fn main() {
    let mut input = String::new();
    println!("Enter a number to convert to Roman Numerals:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please type a number!");
    println!("{} in Roman Numerals is {}", input, roman_numerals(input));
}