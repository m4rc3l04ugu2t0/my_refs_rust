use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path_file: String) {
    let file_result = File::open(path_file);

    let file = match file_result {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut unique_words: HashSet<String> = HashSet::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line_content) => line_content,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        let split_point: Vec<&str> = line.split(", ").collect();
        let joined_line = split_point.join(" ");
        // println!("{:?}", joined_line);
        let split_point: Vec<&str> = joined_line.split(". ").collect();
        let joined_line = split_point.join(" ");
        let split_point: Vec<&str> = joined_line.split(".").collect();
        let joined_line = split_point.join(" ");
        println!("{:?}", joined_line);
        let words: Vec<&str> = joined_line.trim().split_whitespace().collect();
        println!("{:?}", words);

        for word in words {
            let lowercase_word = word.to_lowercase();
            if unique_words.contains(&lowercase_word) {
                unique_words.remove(&lowercase_word);
            } else {
                unique_words.insert(lowercase_word.clone());
            }
        }
    }

    println!("Total de palavras únicas: {}", unique_words.len());
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    read_file(String::from("src/text.txt"));
    // if args.len() > 1 {
    //     match &args[1][..] {
    //         "read_file" => read_file(&args[2]),
    //         _ => println!("ah"),
    //     };
    // } else {
    //     return println!("Error");
    // }
}
