use itertools::Itertools;

use crate::template::{aoc_cli, Day, ANSI_BOLD, ANSI_RESET};
use std::process;

fn example_file_path(day: Day) -> String {
    format!("data/examples/{day}.txt")
}

fn example_file_path_part(day: Day, part: u8) -> String {
    format!("data/examples/{day}-{part}.txt")
}

fn extract_examples(text: &str) -> Vec<&str> {
    text.split("```\n").skip(1).step_by(2).collect_vec()
}

pub fn handle(day: Day) {
    println!("Extracting day {}...", day);
    let text = std::fs::read_to_string(aoc_cli::get_puzzle_path(day)).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });
    let examples = extract_examples(&text);
    let mut selected = Vec::new();
    if examples.len() == 1 {
        selected = examples.clone();
    } else {
        for (i, block) in examples.iter().enumerate() {
            println!("Extracted block {}", i + 1);
            println!("--------------------");
            println!("{ANSI_BOLD}{}{ANSI_RESET}", block.trim());
            println!("--------------------");
            println!("Write this block to a file? [y/N/q]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "y" {
                selected.push(block);
            } else if input.trim().to_lowercase() == "q" {
                println!("Aborted.");
                process::exit(0);
            }
        }
    }
    if selected.len() == 1 {
        let path = example_file_path(day);
        std::fs::write(&path, selected[0]).unwrap_or_else(|err| {
            eprintln!("Error writing file: {}", err);
            process::exit(1);
        });
        println!("🎄 Successfully wrote example to \"{}\".", &path);
    } else {
        for (i, out) in selected.iter().enumerate() {
            let path = example_file_path_part(day, (i + 1) as u8);
            std::fs::write(&path, out).unwrap_or_else(|err| {
                eprintln!("Error writing file: {}", err);
                process::exit(1);
            });
            println!("🎄 Successfully wrote example to \"{}\".", &path);
        }
    }
}
