mod lexer;
mod interpreter;

use std::env;
use std::fs;
use std::io::{self, Read};
use atty::Stream;


const VERSION: &str = env!("CARGO_PKG_VERSION");


fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
   if args.len() > 1 {
		// The GulfOfMexico spec does not mention anything about supporting command line arguments
		eprintln!("Error: too many arguments. Use -h for help.");
		std::process::exit(1);
   }
	else if args.len() < 1 {
		if !atty::is(Stream::Stdin) {
			let mut piped_input = String::new();
			let result = io::stdin().read_to_string(&mut piped_input);
			match result {
				Ok(_len) => {
					let tokens = lexer::parse_string(piped_input);
					interpreter::interpret(tokens);

					std::process::exit(0); // Don't continue to the code below
				}
				Err(err) => {
					eprintln!("Error reading from stdin: {}", err);
				}
			}
		}
		else {
			eprintln!("Error: requires one filename input. Use -h for help.");
			std::process::exit(1);
		}
	}
	else if args.get(0).map(|h_string| ["-h", "--help"].contains(&h_string.as_str())).unwrap_or(false) {
		println!(
"\
GulfOfMexico interpreter by Zakarya, version {}
A 100% spec compliant interpreter for Gulf Of Mexico.

Usage: gulfofmexico [OPTIONS|INPUT]

Options:
    -h, --help       Display this message
    -v, --version    Display version information
    -l, --license    Display license information\
", VERSION);
		std::process::exit(0);
	}
	else if args.get(0).map(|h_string| ["-v", "--version"].contains(&h_string.as_str())).unwrap_or(false) {
		println!(
"\
GulfOfMexico interpreter by Zakarya, version {}\
", VERSION);
		std::process::exit(0);
	}
    else if args.get(0).map(|h_string| ["-l", "--license"].contains(&h_string.as_str())).unwrap_or(false) {
        println!(
"\
GulfOfMexico interpreter version {}
Copyright (C) 2025 Zakarya

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.\
", VERSION);
		std::process::exit(0);
    }

	match fs::read_to_string(&args[0]) {
		Ok(contents) => {
			let tokens = lexer::parse_string(contents);
			interpreter::interpret(tokens);
		}
		Err(err) => {
			eprintln!("Error: failed to read file \"{}\": {}", &args[0], err);
			std::process::exit(1);
		}
	}
}

