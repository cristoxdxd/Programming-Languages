// Binary search program

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number to search for:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let search_for: i32 = input.trim().parse().expect("Please type a number!");

    let mut input = String::new();
    println!("Enter a list of numbers separated by spaces:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    numbers.sort();

    let mut low = 0;
    let mut high = numbers.len() - 1;
    let mut found = false;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = numbers[mid];
        if guess == search_for {
            println!("Found {} at index {}", search_for, mid);
            found = true;
            break;
        } else if guess > search_for {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    if !found {
        println!("{} was not found in the list", search_for);
    }
}