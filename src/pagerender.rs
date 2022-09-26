use crate::mdparser::{MDParser, Token};

pub struct PageRenderer<'a> {
	text: &'a str,
}

impl<'a> PageRenderer<'a> {
	pub fn new(text: &'a str) -> Self {
		PageRenderer { text: text }
	}

	fn render_heading(size: i8, text: &str) -> String {
		format!("<h{size}>{text}</h{size}>\n")
	}

	fn render_blockquote(elements: Vec<Token>) -> String {
		format!("")
	}

	fn render_paragraph(elements: Vec<Token>) -> String {
		format!("")
	}

	pub fn get_html(&self) -> String {
		let mut html = String::new();
		html.push_str("!<doctype html><head>");

		html.push_str("</head>");
		html.push_str("<body><pre>");
		for token in MDParser::new(self.text) {
			let render = match token {
				Token::Heading(size, text) => Self::render_heading(size, text),
				Token::Paragraph(elements) => Self::render_paragraph(elements),
				Token::Blockquote(elements) => Self::render_blockquote(elements),
				Token::Image { src } => todo!(),
				Token::Codeblock { lang, text } => todo!(),
				_ => unreachable!("This tag should not be top level! {:?}", token),
			};
			html.push_str(render.as_str())
		}
		html.push_str("</pre></body>");
		html
	}
}
