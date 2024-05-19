use std::io::{self, Write};

mod functions;

fn main() {

    println!("Choose a converter");
    println!("1. Pound-kg");
    println!("2. Inch-cm");

    let mut converter_choice = String::new();
    io::stdin().read_line(&mut converter_choice).expect("Failed to read the line");

    let parse_choice: i8 = match converter_choice.trim().parse() {
        Ok(num) => num,
        Err(_)=> {
            println!("Invalid input. Please enter a number");
            return;
        }
    };

    match parse_choice {
        1 => {
            functions::pound_kg();
        }
        2 => {
            functions::inch_cm();
        }
        _ => {
            println!("Invalid input")
        }
    };

    // Prompt the user to press Enter to exit
    println!("Press Enter to exit...");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

    // Wait for user input (Enter key)
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
