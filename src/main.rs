//TODO: Remove once out of early prototyping
#![allow(dead_code)]
mod mdparser;

use mdparser::MDParser;

fn main() {
	let parser = MDParser::parse(
		"# This is my level 1 header
## This is my level 2 header

Wow!!!
This is a test paragraph!
*and* a continuation

Second Paragraph",
	);

	for token in parser {
		println!("{:?}", token);
	}
}

#[cfg(test)]
mod tests {
	use crate::mdparser::{MDParser, Token};

	// It is my belief that tests have no need to be pretty, they are far too functional for that.
	#[test]
	fn header_parsing() {
		assert_eq!(
			vec![
				Token::Heading(1, "Test".to_string()),
				Token::Heading(2, "Test".to_string()),
				Token::Heading(3, "Test".to_string()),
				Token::Heading(4, "Test".to_string()),
				Token::Heading(5, "Test".to_string()),
				Token::Heading(6, "Test".to_string()),
			],
			MDParser::parse(
				"# Test
				## Test
				### Test
				#### Test
				##### Test
				###### Test
				"
			)
			.into_iter()
			.collect::<Vec<Token>>()
		);
	}

	#[test]
	fn paragraph_parsing() {
		assert_eq!(
			vec![Token::Paragraph(vec![Token::Text("Hi".to_string())])],
			MDParser::parse("Hi").into_iter().collect::<Vec<Token>>()
		);
	}
}
