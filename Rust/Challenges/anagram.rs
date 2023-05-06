// Given a word and a list the program will return a possible anagram
// from the list of words.

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut word = String::new();
    let mut list = String::new();
    let mut file = File::open("words.txt").expect("File not found");
    file.read_to_string(&mut list).expect("Could not read file");
    println!("Enter a word: ");
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word = word.trim();
    let mut anagram = HashMap::new();
    for line in list.lines() {
        let mut sorted = line.chars().collect::<Vec<char>>();
        sorted.sort();
        let sorted = sorted.into_iter().collect::<String>();
        anagram.entry(sorted).or_insert(Vec::new()).push(line);
    }
    let mut sorted = word.chars().collect::<Vec<char>>();
    sorted.sort();
    let sorted = sorted.into_iter().collect::<String>();
    match anagram.get(&sorted) {
        Some(list) => println!("Anagram: {}", list[0]),
        None => println!("No anagram found"),
    }
}