#[derive(Debug, PartialEq)]
pub enum Token<'a> {
	Heading(i8, &'a str), // Heading size, then the text.
	Paragraph(Vec<Token<'a>>),
	Blockquote(Vec<Token<'a>>),
	Link { href: &'a str, text: &'a str },
	Image { src: &'a str },
	InlineCode(&'a str),
	Codeblock { lang: &'a str, text: &'a str },
	Text(&'a str),
}

pub struct MDParser<'a> {
	markdown: &'a str,
}

impl<'text> MDParser<'text> {
	pub fn new(text: &'text str) -> Self {
		MDParser { markdown: text }
	}
}

impl<'text> Iterator for MDParser<'text> {
	type Item = Token<'text>;

	fn next(&mut self) -> Option<Token<'text>> {
		self.markdown = self.markdown.trim_start();
		if let Some(leading_char) = self.markdown.chars().next() {
			let token = match leading_char {
				'>' => self.consume_blockquote(),
				'#' => self.consume_header(),
				_ => self.consume_paragraph(),
			};
			Some(token)
		} else {
			None
		}
	}
}

impl<'text> MDParser<'text> {
	fn advance_markdown(&mut self, lookahead: usize) {
		self.markdown = &self.markdown[lookahead..];
	}

	fn consume_blockquote(&mut self) -> Token<'text> {
		self.markdown = &self.markdown[1..];
		let mut lookahead = self.markdown.len();

		let mut peek_iter = self.markdown.char_indices().peekable();
		while let Some((index, c)) = peek_iter.next() {
			if c == '\n' {
				if let Some((_, '\n')) = peek_iter.peek() {
					lookahead = index;
					break;
				}
			}
		}

		let text_split = self.markdown[..lookahead]
			.trim()
			.split("\n>")
			.map(str::trim)
			.map(Token::Text);

		self.advance_markdown(lookahead);

		Token::Blockquote(text_split.collect())
	}

	fn consume_header(&mut self) -> Token<'text> {
		// TODO: Split Header case?
		let mut header_level = 0;
		while self.markdown[header_level..].starts_with('#') {
			header_level += 1;
		}

		let lookahead = self.markdown.find('\n').unwrap_or(self.markdown.len());
		let head = &self.markdown[header_level..lookahead];

		self.advance_markdown(lookahead);

		Token::Heading(header_level as i8, head.trim())
	}

	fn try_consume_link(text: &'text str) -> Option<(Token<'text>, usize)> {
		if let Some(close_bracket_location) = text.find(']') {
			if let Some('(') = text[close_bracket_location + 1..].chars().next() {
				if let Some(close_paren_location) = text[close_bracket_location + 1..].find(')') {
					return Some((
						Token::Link {
							href: &text[1..close_bracket_location],
							text: &text[close_bracket_location + 2
								..close_bracket_location + 1 + close_paren_location],
						},
						close_bracket_location + close_paren_location + 2,
					));
				}
			}
		}
		None
	}

	fn consume_paragraph(&mut self) -> Token<'text> {
		let mut lookahead = self.markdown.len();
		let mut elements = Vec::new();
		let mut peek_iter = self.markdown.char_indices().peekable();
		while let Some((index, c)) = peek_iter.next() {
			if c == '\n' {
				if let Some((_, '\n')) = peek_iter.peek() {
					lookahead = index;
					break;
				}
			}
			if c == '[' {
				if let Some((link, lookahead)) = Self::try_consume_link(&self.markdown[index..]) {
					elements.extend(
						self.markdown[..index]
							.trim()
							.split('\n')
							.map(str::trim)
							.map(Token::Text),
					);
					self.advance_markdown(index + lookahead);
					peek_iter = self.markdown.char_indices().peekable();
					println!("{:?}", link);
					elements.push(link);
					println!("{}", &self.markdown);
				}
			}
		}
		elements.extend(
			self.markdown[..lookahead]
				.trim()
				.split('\n')
				.map(str::trim)
				.map(Token::Text),
		);

		self.advance_markdown(lookahead);

		Token::Paragraph(elements)
	}
}

#[cfg(test)]
mod tests {

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
			vec![Token::Blockquote(vec![
				Token::Text("Quote"),
				Token::Text("Quote line 2"),
			])],
			"> Quote\n> Quote line 2",
		)
	}
}
