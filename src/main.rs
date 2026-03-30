#![allow(unused_variables)]
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                let mut line_no = 0;
                for line in file_contents.lines() {
                    line_no += 1;
                    for c in line.chars(){
                        match c {
                            '(' => println!("LEFT_PAREN ( null"),
                            ')' => println!("RIGHT_PAREN ) null"),

                            '{' => println!("LEFT_BRACE {{ null"),
                            '}' => println!("RIGHT_BRACE }} null"),

                            '.' => println!("DOT . null"),
                            ',' => println!("COMMA , null"),
                            ';' => println!("SEMICOLON ; null"),

                            '+' => println!("PLUS + null"),
                            '-' => println!("MINUS - null"),
                            '*' => println!("STAR * null"),
                            '/' => println!("SLUSH / null"),

                            _ => eprintln!("[line {}] Error: Unexpected character: {}", line_no, c),
                        }
                    }

                }
                println!("EOF  null");
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
