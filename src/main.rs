use colored::*;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct InforFile {
    total_words: u32,
    total_characters: usize,
    total_phrases: u16,
    total_paragraphs: u8,
    characters_without_spaces: usize,
    unique_words: u8,
}

fn print_result(
    InforFile {
        total_words,
        total_characters,
        total_phrases,
        total_paragraphs,
        characters_without_spaces,
        unique_words,
    }: InforFile,
) {
    println!("{:-^50}", " Contador de Palavras ");
    println!(
        "\
        Total de palavras únicas: {}\n\
        Total de parágrafos: {}\n\
        Total de palavras: {}\n\
        Total de frases: {}\n\
        Caracteres(sem quebra de linha): {}\n\
        Caracteres sem espaços: {}",
        unique_words.to_string().green(),
        total_paragraphs.to_string().green(),
        total_words.to_string().green(),
        total_phrases.to_string().green(),
        total_characters.to_string().green(),
        characters_without_spaces.to_string().green()
    );
    println!("{:-^50}", " &NextLevelCode ");
}

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
    let mut total_words: u32 = 0;
    let mut total_characters: usize = 0;
    let mut total_phrases: u16 = 0;
    let mut total_paragraphs: u8 = 0;
    let mut characters_without_spaces: usize = 0;

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
        total_phrases += split_point.len() as u16;
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

    let infor_file = InforFile {
        characters_without_spaces,
        total_paragraphs,
        total_phrases,
        total_characters,
        total_words,
        unique_words: unique_words.len() as u8,
    };

    print_result(infor_file);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match &args[1][..] {
            "read_file" => read_file(&args[2]),
            _ => println!("Comando invalido!"),
        };
    } else {
        return println!("Error");
    }
}
