// This file implements a command-line interface for displaying the options data.
// It exports functions like display_terminal and get_user_input.

use std::io::{self, Write};

pub fn display_terminal(options_data: &str) {
    println!("Real-Time Options Data:");
    println!("{}", options_data);
}

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}