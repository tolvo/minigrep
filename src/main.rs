use std::env;
use std::process;

use minigrep::Config;

fn main() {
	let args: Vec<String> = env::args().collect();

	let config  = Config::build(&args).unwrap_or_else(|_err| {
		process::exit(1);
	});

	if let Err(_e) = minigrep::run(config) {
		process::exit(1);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, contents))
	}
}
