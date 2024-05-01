use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::*;

fn read_file(path_file: &String) {
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
    let mut total_paragraphs = 0;
    let mut total_phrases = 0;
    let mut characters_without_spaces = 0;
    let mut total_characters = 0;
    let mut total_words = 0;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line_content) => line_content,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        total_paragraphs += 1;
        total_characters += line.chars().count();

        let split_point: Vec<&str> = line.split(", ").collect();
        let joined_line = split_point.join(" ");
        let split_point: Vec<&str> = joined_line.split(". ").collect();
        total_phrases += split_point.len();
        let joined_line = split_point.join(" ");
        let split_point: Vec<&str> = joined_line.split(".").collect();
        let joined_line = split_point.join(" ");
        let split_point: Vec<&str> = joined_line.split("!").collect();
        let joined_line = split_point.join(" ");
        let words: Vec<&str> = joined_line.trim().split_whitespace().collect();

        for word in words {
            let lowercase_word = word.to_lowercase();
            characters_without_spaces += word.to_string().chars().count();
            total_words += 1;
            if unique_words.contains(&lowercase_word) {
                unique_words.remove(&lowercase_word);
            } else {
                unique_words.insert(lowercase_word.clone());
            }
        }
    }

    println!("{:-^50}", " Contador de Palavras ");
    println!(
        "\
        Total de palavras únicas: {}\n\
        Total de parágrafos: {}\n\
        Total de palavras: {}\n\
        Total de frases: {}\n\
        Caracteres(sem quebra de linha): {}\n\
        Caracteres sem espaços: {}",
        unique_words.len().to_string().green(),
        total_paragraphs.to_string().green(),
        total_words.to_string().green(),
        total_phrases.to_string().green(),
        total_characters.to_string().green(),
        characters_without_spaces.to_string().green()
    );
    println!("{:-^50}", " &NextLevelCode ");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match &args[1][..] {
            "read_file" => read_file(&args[2]),
            _ => println!("ah"),
        };
    } else {
        return println!("Error");
    }
}
