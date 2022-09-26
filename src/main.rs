//TODO: Remove once out of early prototyping
#![allow(dead_code)]
mod mdparser;

use mdparser::MDParser;

fn main() {
	let parser = MDParser::new(
		"# This is my level 1 header
## This is my level 2 header

> Blockquote
> Blockquote pt.ii

> Second Blockquote

Wow!!!
This is a test paragraph!
*and* a continuation

Second Paragraph",
	);

	// Take for testing in case i accidentally make infinite tags...
	for token in parser.take(10) {
		println!("{:?}", token);
	}
}

#[cfg(test)]
mod tests {
	use std::vec;

	use crate::mdparser::{MDParser, Token};

	fn assert_parse(parse: Vec<Token>, md: &str) {
		assert_eq!(parse, MDParser::new(md).collect::<Vec<Token>>());
	}

	// It is my belief that tests have no need to be pretty, they are far too functional for that.
	#[test]
	fn all_header_sizes_should_be_valid() {
		assert_parse(vec![Token::Heading(1, "Test")], "# Test");
	}

	#[test]
	fn adjacent_paragraphs_should_parse_seperately() {
		assert_parse(
			vec![Token::Heading(1, "test"), Token::Heading(2, "test2")],
			"# test\n## test2",
		)
	}

	// TODO: test for multiple headers directly next to each other.
	// The behavior I want is significantly harder to code than the other natural option.

	#[test]
	fn unmarked_lines_are_paragraphs() {
		assert_parse(vec![Token::Paragraph(vec![Token::Text("Hi")])], "Hi");
	}

	#[test]
	fn paragraphs_with_blank_line_between_parse_seperately() {
		assert_parse(
			vec![
				Token::Paragraph(vec![Token::Text("Paragraph 1")]),
				Token::Paragraph(vec![Token::Text("Paragraph 2")]),
			],
			"Paragraph 1

Paragraph 2",
		);

		assert_parse(
			vec![
				Token::Paragraph(vec![Token::Text("Paragraph 1")]),
				Token::Paragraph(vec![Token::Text("Paragraph 2")]),
			],
			"Paragraph 1


Paragraph 2",
		);
	}

	#[test]
	fn adjacent_paragraphs_should_merge() {
		assert_parse(
			vec![Token::Paragraph(vec![
				Token::Text("Paragraph"),
				Token::Text("Continuation"),
			])],
			"Paragraph
		Continuation",
		)
	}

	#[test]
	fn lines_marked_right_bracket_parse_as_blockquote() {
		assert_parse(
			vec![Token::Blockquote(vec![Token::Text("Quote")])],
			"> Quote",
		)
	}

	#[test]
	fn adjacent_blockquotes_merge_to_one() {
		assert_parse(
			vec![Token::Blockquote(vec![Token::Text("Quote"), Token::Text("Quote line 2")])],
			"> Quote\n> Quote line 2",
		)
	}
}
