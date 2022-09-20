#[derive(Debug)]
pub enum Token<'a> {
	Heading(i8, &'a str), // Heading size, then the text.
	Paragraph(Vec<Token<'a>>),
	Blockquote(&'a str),
	Link { href: &'a str, text: &'a str },
	Codeblock { lang: &'a str, text: String },
	Text (String),
	Done,
}

pub struct MDParser<'a> {
	markdown: &'a str,
	index: usize,
}

impl MDParser<'_> {

	pub fn parse(text: &str) -> MDParser{
		MDParser {
			markdown: text,
			index: 0,
		}
	}

	fn get_leading_grapheme(text: &str) -> &str {
		if let Some(back_index) = text.find(char::is_whitespace){
			&text[..back_index]
		}else{
			""
		}
	}

	pub fn next(&mut self) -> Token {
		if let Some(index) = self.markdown[self.index..].find(|c: char| !c.is_whitespace()){
			self.index = index;
		}else{
			return Token::Done;
		}
		let leading_grapheme = MDParser::get_leading_grapheme(&self.markdown[self.index..]);

		match leading_grapheme{
			_ => self.consume_paragraph()
		}
	}

	fn consume_paragraph(&mut self) -> Token {
		let start = self.index;
		// TODO: remove the need to both iterate for find and then iterate in the replace.

		if let Some(end) = self.markdown[self.index..].find("\n\n"){
			self.index = end;
			MDParser::parse_paragraph_internals(&self.markdown[start..end])
		}else{
			// I think this should be unreachable in normal operation
			MDParser::parse_paragraph_internals(&self.markdown[start..])
		}
	}

	fn parse_paragraph_internals(text: &str)-> Token {
		let contents: Vec<Token> = Vec::new();
		while text.len() > 0 { // We will slowly consume our reference
			if let Some(token_index) = text.find(['*','\\','[','_']){
				match text[token_index] {
					'*' => (),
					'\\' => (),
					'[' => (),
					'_' => (),
				}
			}
		}
		Token::Paragraph(contents)
	}

}
