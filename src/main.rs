mod elements;
mod parser;
use io::Write;
use std::io;

use parser::Parser;
fn main() {
    let mut buffer = String::new();

    println!("Bitte gib die Strukturformel ein:");
    print!("> ");
    io::stdout().flush().expect("Could not flush the terminal");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read in the input");
    let mut parser = Parser::new(buffer.trim().to_string());
    let tokens = parser.parse();
    let elements = parser.convert_tokens_to_elements(tokens);
    elements
        .iter()
        .for_each(|x| println!("{} x{} \n", x.0, x.1));
    let atomare_masse_gesamt: f64 = elements.iter().map(|x| x.0.atomaremasse * x.1 as f64).sum();

    println!("{} g/mol", atomare_masse_gesamt);
}
