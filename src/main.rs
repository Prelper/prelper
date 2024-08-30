// fn main() {
//     println!("Hello, world!");
// }




use std::env;
use std::io::{self, Write};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    let input_code: String;

    // Check if -s argument is provided
    if args.len() > 2 && args[1] == "-s" {
        input_code = args[2].clone();
    } else {
        // Prompt the user to enter a string
        print!("Please enter the code snippet: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input_code = input.trim().to_string();
    }

    // Guess the programming language
    let language = guess_language(&input_code);
    println!("Guessed Language: {}", language);

    // Provide a basic explanation of the code
    let explanation = explain_code(&input_code, &language);
    println!("Code Explanation: {}", explanation);
}

fn guess_language(code: &str) -> &str {
    if code.contains("fn main()") {
        "Rust"
    } else if code.contains("def ") {
        "Python"
    } else if code.contains("public static void main") {
        "Java"
    } else if code.contains("#include <iostream>") {
        "C++"
    } else if code.contains("console.log") {
        "JavaScript"
    } else if code.contains("print") {
        "Python"
    } else if code.contains("class ") {
        "Java"
    } else {
        "Unknown"
    }
}

fn explain_code(code: &str, language: &str) -> String {
    match language {
        "Rust" => "This appears to be a Rust program. Rust is a systems programming language focused on safety and performance.".to_string(),
        "Python" => "This appears to be a Python script. Python is an interpreted, high-level, general-purpose programming language.".to_string(),
        "Java" => "This appears to be a Java program. Java is a high-level, class-based, object-oriented programming language.".to_string(),
        "C++" => "This appears to be a C++ program. C++ is a general-purpose programming language created as an extension of the C programming language.".to_string(),
        "JavaScript" => "This appears to be a JavaScript code. JavaScript is a programming language commonly used in web development.".to_string(),
        _ => "Unable to determine the programming language.".to_string(),
    }
}