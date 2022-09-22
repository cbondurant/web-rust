#[derive(Debug, PartialEq)]
pub enum Token<'a> {
	Heading(i8, &'a str), // Heading size, then the text.
	Paragraph(Vec<Token<'a>>),
	Blockquote(&'a str),
	Link { href: &'a str, text: &'a str },
	Codeblock { lang: &'a str, text: &'a str },
	Text(String),
	Done,
}

pub struct MDParser<'a> {
	markdown: &'a str,
	// TODO: what is the + 'a syntax on display here?
	// It was important to this block working but I don't understand it yet.
	blocks: Box<dyn Iterator<Item = &'a str> + 'a>,
}

impl<'a> IntoIterator for MDParser<'a> {

	type Item = Token<'a>;
	type IntoIter = MDParserIter<'a>;

	fn into_iter(self) -> Self::IntoIter {

	}
}

struct MDParserIter<'parser> {
	parser: MDParser<'parser>,
	blocks: Box<dyn Iterator<Item = &'parser str> + 'parser>,
}

impl<'parser> Iterator for MDParserIter<'parser> {
	type Item = Token<'parser>;

	fn next(&mut self) -> Option<Token> {
		if let Some(body) = self.blocks.next() {
			if body == "" { return None }
			let leading_char = &body[..1];

			match leading_char {
				// Theres gotta be a better way here but hell if I know what it is.
				"#" => Some(Self::consume_header(body)),
				_ => Some(Self::consume_paragraph(body)),
			}
		} else {
			return None;
		}
	}
}

impl<'a> MDParser<'a> {
	pub fn parse(text: &str) -> MDParser {
		MDParser {
			markdown: text,
			blocks: Box::new(text.split("\n\n")),
		}
	}

	fn consume_header(mut text: &str) -> Token {
		let mut header_level = 0;
		while Some("#") == text.get(..1) {
			header_level += 1;
			text = &text[1..];
		}

		text = &text[text.find(|c: char| !c.is_whitespace()).unwrap_or(0)..];

		Token::Heading(header_level, text.trim())
	}

	fn consume_paragraph(text: &str) -> Token {
		let mut contents = Vec::new();

		let mut text_split = text.trim().split('\n');

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
