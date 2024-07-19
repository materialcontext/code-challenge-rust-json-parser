use std::env;
use std::fs;
use parse_json::json::{lexer::Tokenizer, parser::Parser};

#[derive(Debug)]
struct Name {
    field: word
}

fn main() {
    // Get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // Check if a file path was provided.
    if args.len() < 2 {
        println!("Please provide a file path.");
        return;
    }

    // Read the file.
    let file_contents = fs::read_to_string(&args[1]);
    let file_contents = match file_contents {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error reading file: {}", error);
            return;
        }
    };


    let tokens = Tokenizer::new(&file_contents).tokenize();
    let parser = Parser::new(tokens.unwrap()).parse();


    match parser {
        Ok(_) => println!("This is valid JSON. Great!"),
        Err(err) => println!("{}", err)
    }
}
