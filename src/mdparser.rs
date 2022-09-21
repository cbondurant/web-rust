#[derive(Debug, PartialEq)]
pub enum Token {
	Heading(i8, String), // Heading size, then the text.
	Paragraph(Vec<Token>),
	Blockquote(String),
	Link { href: String, text: String },
	Codeblock { lang: String, text: String },
	Text(String),
	Done,
}

pub struct MDParser<'a> {
	markdown: &'a str,
	index: usize,
}

impl<'a> Iterator for MDParser<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		if let Some(index) = self.markdown[self.index..].find(|c: char| !c.is_whitespace()) {
			self.index += index; // Index is relative now
		} else {
			return None;
		}
		let leading_grapheme = MDParser::get_leading_grapheme(&self.markdown[self.index..]);

		// Mutate the index internally here, instead of having to manage passing it in and out of the
		// function call
		match leading_grapheme {
			// Theres gotta be a better way here but hell if I know what it is.
			"#" => Some(self.consume_header()),
			"##" => Some(self.consume_header()),
			"###" => Some(self.consume_header()),
			"####" => Some(self.consume_header()),
			"#####" => Some(self.consume_header()),
			"######" => Some(self.consume_header()),
			_ => Some(self.consume_paragraph()),
		}
	}
}

impl<'a> MDParser<'a> {
	pub fn parse(text: &str) -> MDParser {
		MDParser {
			markdown: text,
			index: 0,
		}
	}

	fn get_leading_grapheme(text: &str) -> &str {
		if let Some(back_index) = text.find(char::is_whitespace) {
			&text[..back_index]
		} else {
			""
		}
	}

	fn consume_header(&mut self) -> Token {
		let mut header_level = 0;
		while Some("#") == self.markdown[self.index..].get(..1) {
			header_level += 1;
			self.index += 1;
		}

		self.index += self.markdown[self.index..]
			.find(|c: char| !c.is_whitespace())
			.unwrap_or(0);

		let start = self.index;
		// I think headers should only be one line.
		let end = self.markdown[self.index..]
			.find('\n')
			.unwrap_or_else(|| self.markdown[self.index..].len());
		self.index += end;
		Token::Heading(header_level, self.markdown[start..start + end].to_string())
	}

	fn consume_paragraph(&mut self) -> Token {
		let start = self.index;
		// TODO: remove the need to both iterate for find and then iterate in the replace.

		if let Some(end) = self.markdown[self.index..].find("\n\n") {
			self.index += end;
			MDParser::parse_paragraph_internals(&self.markdown[start..start + end])
		} else {
			self.index = self.markdown.len(); // This paragraph is the final bit of text, reached end of text.
			MDParser::parse_paragraph_internals(&self.markdown[start..])
		}
	}

	fn parse_paragraph_internals(text: &str) -> Token {
		let mut contents = Vec::new();

		let mut text_split = text.split('\n');

		let mut new_text = String::new();
		if let Some(start) = text_split.next() {
			new_text.push_str(start);

			for line in text_split {
				new_text.push(' ');
				new_text.push_str(line.trim());
			}
		}
		let body = Token::Text(new_text);
		contents.push(body);
		Token::Paragraph(contents)
	}
}
