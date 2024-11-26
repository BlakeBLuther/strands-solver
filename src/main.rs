use std::fs;
use std::path::Path;
use std::io::{self, BufRead};

use crate::strands::Strands;

mod trie;
mod strands;

fn main() {
    let file_path = Path::new("./english-words/words_alpha_pruned.txt"); //using a custom dict. just english words with len <4 removed.
    println!("Reading file {}...", file_path.to_str().unwrap());
    let file = fs::File::open(file_path).expect("Failed to open file.");
    let reader = io::BufReader::new(file);

    let mut trie = trie::Trie::new();
    for line in reader.lines() {
        let line = line.expect("Error reading line.");
        trie.insert(line);
    }
    println!("Trie initialized.");

    let file_path = Path::new("./strands.txt");
    println!("Reading file {}...", file_path.to_str().unwrap());
    let puzzle = fs::read_to_string(file_path).expect("Unable to read puzzle.");
    let strands = Strands::new(puzzle, 8);
    println!("Puzzle loaded:");
    strands.print();
    println!("Attempting to solve...");
    if let Some(results) = strands.solve(&trie) {
        println!("Solution:\n");
        for row in results.keys() {
            println!("{}", row);
        }
    } else {
        print!("No solution found!");
    }

}
