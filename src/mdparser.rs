#[derive(Debug, PartialEq)]
pub enum Token<'a> {
	Heading(i8, &'a str), // Heading size, then the text.
	Paragraph(Vec<Token<'a>>),
	Blockquote(&'a str),
	Link { href: &'a str, text: &'a str },
	// Codeblock { lang: &'a str, text: &'a str },
	Text(&'a str),
	Done,
}

pub struct MDParser<'a> {
	markdown: &'a str,
}

pub struct MDParserIter<'parser, 'text> {
	parser: &'parser MDParser<'text>,
	blocks: Box<dyn Iterator<Item = &'text str> + 'text>,
}

impl<'parser, 'text> MDParserIter<'parser, 'text> {
	fn new(parser: &'parser MDParser<'text>) -> Self {
		MDParserIter {
			parser: parser,
			blocks: Box::new(parser.markdown.split("\n\n")),
		}
	}

	fn consume_header(mut text: &str) -> Token {
		// TODO: Split Header case?
		let mut header_level = 0;
		while Some("#") == text.get(..1) {
			header_level += 1;
			text = &text[1..];
		}

		text = &text[text.find(|c: char| !c.is_whitespace()).unwrap_or(0)..];

		Token::Heading(header_level, text.trim())
	}

	fn consume_paragraph(text: &str) -> Token {

		let text_split = text.trim().split('\n').map(str::trim).map(Token::Text);

		Token::Paragraph(text_split.collect())
	}
}

impl<'text> Iterator for MDParserIter<'_, 'text> {
	type Item = Token<'text>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(body) = self.blocks.next() {
			if body == "" {
				return None; // TODO: This breaks it weird.
			}
			let leading_char = &body.trim()[..1];

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

impl<'parser, 'text> MDParser<'text> {
	pub fn parse(text: &str) -> MDParser {
		MDParser { markdown: text }
	}

	pub fn iter(&'parser self) -> MDParserIter<'parser, 'text> {
		MDParserIter::new(self)
	}
}
