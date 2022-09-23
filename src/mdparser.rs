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
	Done,
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
			.split('\n')
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

	fn consume_paragraph(&mut self) -> Token<'text> {
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
			.split('\n')
			.map(str::trim)
			.map(Token::Text);

		self.advance_markdown(lookahead);

		Token::Paragraph(text_split.collect())
	}
}
