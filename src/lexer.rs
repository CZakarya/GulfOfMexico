pub const BEFORE_SYMBOLS: [char; 12] = ['¤', '$', '£', '€', '¥', '₹', '₽', '₺', '₩', '₪', '₦', '₵'];
pub const MIDDLE_SYMBOLS: [char; 2]  = ['¤', '$'];
pub const AFTER_SYMBOLS:  [char; 5]  = ['¤', '¢', '₽', '₫', '€'];


pub enum TokenType {
	StringLiteral,
	StringInterp,
}

pub struct Token {
	pub t_type: TokenType,
	pub value: String,
	pub line_number: u16,
}


fn slice_chars_inclusive(chars: &[char], start: usize, end_inclusive: usize) -> String {
	if start > end_inclusive || end_inclusive >= chars.len() { return String::new(); }
	chars[start..=end_inclusive].iter().collect()
}

pub fn parse_string(file: String) -> Vec<Token> {
	let file_chars: Vec<char> = file.chars().collect();

	let mut tokens: Vec<Token> = vec!();

	let mut state_token_start: i32 = 0;
	let mut state_token_end: i32 = 0; // Inclusive
	let mut state_found_open_brace: i32 = -1;
	let mut state_found_middle_symbol: i32 = -1;
	let mut state_found_close_brace: i32;
	let mut state_end_of_interp: i32 = 0;
	let mut state_was_end_symbol: bool = false;
	let mut state_line_number: u16 = 1;

	// Check for string interpolation
	for (iter, &character) in file_chars.iter().enumerate() {
		// Check for newline
		if character == '\n' {
			state_line_number += 1;
			continue;
		}


		if character == '{' {
			state_found_open_brace = iter as i32;
		}
	
		// Check if there is a interpolation indicator in the middle
		else if MIDDLE_SYMBOLS.contains(&character) && state_found_open_brace != -1 {
			state_found_middle_symbol = iter as i32;
			state_token_end = state_found_open_brace - 1;
		}

		else if character == '}' && state_found_open_brace != -1 {
			state_found_close_brace = iter as i32;
			state_end_of_interp = iter as i32;
			let mut interp_found = false;

			if state_found_middle_symbol == -1 {
				// Check if there is a interpolation indicator at the beginning
				let check_char = file_chars[state_found_open_brace as usize - 1];
				if BEFORE_SYMBOLS.contains(&check_char) {
					interp_found = true;
					state_token_end = state_found_open_brace - 2;
				}

				// Check if there is a interpolation indicator at the end
				state_was_end_symbol = false;
				let end_check_char = file_chars[iter + 1];
				if AFTER_SYMBOLS.contains(&end_check_char) {
					interp_found = true;
					state_token_end = state_found_open_brace - 1;
					state_end_of_interp = iter as i32 + 1;
					state_was_end_symbol = true;
				}
			}
			else { // Middle symbol was found
				interp_found = true;
			}

			if interp_found {
				let string_literal_value = slice_chars_inclusive(&file_chars, state_token_start as usize, state_token_end as usize);
				//println!("String literal: {}", string_literal_value);
				tokens.push(Token {
					t_type: TokenType::StringLiteral,
					value: string_literal_value,
					line_number: state_line_number,
				});
				
				let string_interp_value = slice_chars_inclusive(&file_chars, state_found_open_brace as usize + 1, state_found_close_brace as usize - 1);
				tokens.push(Token {
					t_type: TokenType::StringInterp,
					value: string_interp_value,
					line_number: state_line_number,
				});

				// Reset state
				state_token_start = iter as i32 + (if state_was_end_symbol { 2 } else { 1 });
				state_token_end = 0; // Inclusive
				state_found_open_brace = -1;
				state_found_middle_symbol = -1;
				//state_found_close_brace = -1; // Does not need to happen, this value never gets tested
			}
		}
	}

	if state_token_start < file_chars.len() as i32 {
		let string_literal_value = slice_chars_inclusive(&file_chars, state_end_of_interp as usize + 1, file_chars.len() as usize);
		//println!("String literal: {}", string_literal_value);
		tokens.push(Token {
			t_type: TokenType::StringLiteral,
			value: string_literal_value,
			line_number: state_line_number,
		});
	}

	return tokens;
}
