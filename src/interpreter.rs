use crate::lexer::{Token, TokenType, MIDDLE_SYMBOLS};

use std::collections::HashMap;


fn replace_chars_with_dot(input: &str, chars: &[char]) -> String {
	let set: std::collections::HashSet<char> = chars.iter().copied().collect();
	input.chars()
	.map(|character| if set.contains(&character) { '.' } else { character })
	.collect()
}

pub fn interpret(tokens: Vec<Token>) {
	let variables: HashMap<String, String> = HashMap::new();

	for token in tokens {
		match token.t_type {
			TokenType::StringLiteral => {} // Unexpected string literal; ignore
			TokenType::StringInterp => {
				let var_name = replace_chars_with_dot(&token.value, &MIDDLE_SYMBOLS);
				if variables.contains_key(&var_name) {} // Treat as a string
				else {
					eprintln!("NameError on line {}: \"{}\" is undefined.", token.line_number, var_name);
					std::process::exit(1); // Comment this out to go through all of the string interpolations
				}
			}
		}
	}
}
