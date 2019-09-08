/// Huffman compression algorithm in Rust programming language
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn source_statistics(file_path: &str) -> Result<HashMap<char, u32>, std::io::Error> {
    // open and reading the file
    let file = File::open(file_path).expect("error opening the file");
    let reader = BufReader::new(file);
    // TODO(elsuizo:2019-09-07): pero no se si esta bien hacerlo asi porque me parece que se esta
    // evitando los posibles errores
    let lines: Vec<String> = reader.lines().flatten().collect(); // transform the file on Vector of String
    let mut probabilities: HashMap<char, u32> = HashMap::new();
    for line in lines {
        for c in line.chars() {
            *probabilities.entry(c).or_insert(0) += 1; // look if exist the char in the dictionary and add +1 to the count
        }
    }
    Ok(probabilities)
}

fn main() {
    // NOTE(elsuizo:2019-09-07): no se porque los paths relativos no andan(o sea
    // ../Files/castellano.txt)
    let file_path = "/home/elsuizo/Repos/huffman-rs/Files/castellano.txt";
    let mut probs: HashMap<char, u32> = HashMap::new();
    probs = source_statistics(file_path).unwrap();
    println!("{:?}", probs);
}
