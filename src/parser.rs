use std::fmt;

use crate::elements::PERIODEN_SYSTEM;
#[derive(Debug)]
pub struct Element {
    pub name: &'static str,
    pub symbol: &'static str,
    pub atomaremasse: f64,
    pub ordnungszahl: i32,
}
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} ({}) \n Masse: {}",
            self.ordnungszahl, self.name, self.symbol, self.atomaremasse
        )
    }
}

pub struct Token {
    symbol: String,
    count: u32,
}

enum States {
    Start,
    Einfach,
    Zweifach,
    AnzahlIndikator,
    Anzahl,
    Error,
    Done,
}

enum InputTypes {
    Uppercase,
    Lowercase,
    Underline,
    Number,
    Invalid,
    End,
}

pub struct Parser {
    state: States,
    cursor: usize,
    expression: Vec<char>,
}

impl Parser {
    pub fn new(expression: String) -> Parser {
        Self {
            state: States::Start,
            cursor: 0,
            expression: expression.chars().collect(),
        }
    }
    fn get_char(&self) -> char {
        self.expression[self.cursor]
    }
    fn is_done(&self) -> bool {
        self.cursor == self.expression.len()
    }
    fn next(&mut self) {
        self.cursor += 1;
    }

    fn transition(&mut self, input: InputTypes) {
        self.state = match self.state {
            States::Start => match input {
                InputTypes::Uppercase => States::Einfach,
                _ => States::Error,
            },
            States::Einfach => match input {
                InputTypes::Uppercase | InputTypes::End => States::Done,
                InputTypes::Lowercase => States::Zweifach,
                InputTypes::Underline => States::AnzahlIndikator,
                _ => States::Error,
            },
            States::Zweifach => match input {
                InputTypes::Uppercase | InputTypes::End => States::Done,
                InputTypes::Underline => States::AnzahlIndikator,
                _ => States::Error,
            },

            States::Anzahl => match input {
                InputTypes::Number => States::Anzahl,
                _ => States::Done,
            },

            States::AnzahlIndikator => match input {
                InputTypes::Number => States::Anzahl,
                _ => States::Error,
            },

            States::Done => match input {
                InputTypes::Uppercase => States::Einfach,
                InputTypes::End => States::Done,
                _ => States::Error,
            },

            _ => States::Error,
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut symbol = String::new();
        let mut count_string = String::from("1");
        let mut tokens: Vec<Token> = Vec::<Token>::new();

        while !self.is_done() {
            let character = self.get_char();
            if character.is_uppercase() {
                self.transition(InputTypes::Uppercase);
            } else if character.is_lowercase() {
                self.transition(InputTypes::Lowercase);
            } else if character == '_' {
                self.transition(InputTypes::Underline);
            } else if character.is_ascii_digit() {
                self.transition(InputTypes::Number);
            } else {
                self.transition(InputTypes::Invalid);
            }

            match self.state {
                States::Anzahl => count_string.push(character),
                States::AnzahlIndikator => count_string.clear(),
                States::Done => {
                    tokens.push(Token {
                        symbol,
                        count: count_string.parse().expect("Failed to parse String to u32"),
                    });
                    symbol = String::new();
                    count_string = "1".to_string();
                }
                States::Error => {
                    break;
                }
                States::Einfach | States::Zweifach => symbol.push(character),
                _ => {}
            }

            match self.state {
                States::Done => {}
                _ => self.next(),
            }
        }

        self.transition(InputTypes::End);
        match self.state {
            States::Done => tokens.push(Token {
                symbol,
                count: count_string.parse().expect("Failed to parse"),
            }),
            _ => println!("Dies ist keine gültige Formel"),
        }

        tokens
    }

    pub fn convert_tokens_to_elements(&self, tokens: Vec<Token>) -> Vec<(&Element, u32)> {
        let mut elements: Vec<(&Element, u32)> = vec![];
        for token in tokens {
            let element = match PERIODEN_SYSTEM.iter().find(|x| x.symbol == token.symbol) {
                Some(v) => v,
                None => &Element {
                    name: "Unknown",
                    symbol: "None",
                    atomaremasse: 0.0,
                    ordnungszahl: 0,
                },
            };
            elements.push((element, token.count));
        }
        elements
    }
}
