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
			let (token, lookahead) = match leading_char {
				// Theres gotta be a better way here but hell if I know what it is.
				'#' => Self::consume_header(self.markdown),
				_ => Self::consume_paragraph(self.markdown),
			};
			self.markdown = &self.markdown[lookahead..];
			Some(token)
		} else {
			return None;
		}
	}
}

impl<'text> MDParser<'text> {

	fn consume_header(markdown: &'text str) -> (Token<'text>, usize) {
		// TODO: Split Header case?
		let mut header_level = 0;
		while markdown[header_level..].starts_with('#') {
			header_level += 1;
		}

		let eol = markdown.find('\n').unwrap_or(markdown.len());
		let head = &markdown[header_level..eol];


		(Token::Heading(header_level as i8, head.trim()), eol)
	}

	fn consume_paragraph(markdown: &'text str) -> (Token, usize) {

		let mut lookahead = markdown.len();

		let mut peek_iter = markdown.char_indices().peekable();
		while let Some((index, c)) = peek_iter.next(){
			if c == '\n' {
				if let Some((_,'\n')) = peek_iter.peek() {
					lookahead = index;
					break;
				}
			}
		}

		let text_split = markdown[..lookahead].trim().split('\n').map(str::trim).map(Token::Text);

		(Token::Paragraph(text_split.collect()), lookahead)
	}
}