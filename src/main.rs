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
			vec![Token::Heading(1, "Test".to_string())],
			MDParser::parse("# Test")
				.into_iter()
				.collect::<Vec<Token>>()
		);
	}

	// TODO: test for multiple headers directly next to each other.
	// The behavior I want is significantly harder to code than the other natural option.

	#[test]
	fn paragraph_parsing() {
		assert_eq!(
			vec![Token::Paragraph(vec![Token::Text("Hi".to_string())])],
			MDParser::parse("Hi").into_iter().collect::<Vec<Token>>()
		);
	}

	#[test]
	fn multiple_paragraphs() {
		assert_eq!(
			vec![
				Token::Paragraph(vec![Token::Text("Paragraph 1".to_string())]),
				Token::Paragraph(vec![Token::Text("Paragraph 2".to_string())])
			],
			MDParser::parse(
				"Paragraph 1

Paragraph 2"
			)
			.into_iter()
			.collect::<Vec<Token>>()
		);
	}
}
