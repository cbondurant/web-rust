use std::collections::HashMap;

use crate::mdparser::{MDParser, Token};

enum RenderConfigItem {
	Bool(bool),
	Integer(i32),
	String(String),
}

pub struct PageRenderer<'a> {
	text: &'a str,
	configuration: HashMap<&'a str, RenderConfigItem>,
}

impl<'a> PageRenderer<'a> {
	pub fn new(text: &'a str) -> Self {
		let configuration = HashMap::from([("page_width", "80".to_string())]);
		PageRenderer {
			text: text,
			configuration: HashMap::new(),
		}
	}

	fn generate_meta(&self) -> String {
		// Yeah ill make this better later, I need something in place.
		"body {
			background-color:#0c0a10;
			position: relative;
			font-family: 'Courier New', Courier, monospace;
			color: #798699;
			font-size: 15px;
		}
		pre {
			margin:0 auto;
			display:block;
			width:auto;
			text-align:center;
		}
		.grime {
			color:#003172;
		}
		a:link, a:visited {
			color:#7df5ff;
			text-decoration:none;
		}
		a.link:link, a.link:visited {
			color:#fff;
			text-decoration:none;
		}
		a:hover, a:active {
			color:#003172 !important;
			background: none !important;
		}"
		.to_string()
	}

	fn render_heading(size: i8, text: &str) -> String {
		format!("<h{size}>{text}</h{size}>")
	}

	fn render_blockquote(elements: Vec<Token>) -> String {
		let mut html = String::new();
		html.push_str("<p>");
		let mut current_line_length = 0;
		html.push_str("* ");
		for element in elements {
			match element {
				Token::Text(text) => {
					for word in text.split_ascii_whitespace() {
						if word.len() + current_line_length <= 60 {
							if current_line_length != 0 {
								html.push(' ');
							}
							html.push_str(word);

							current_line_length += word.len() + (current_line_length != 0) as usize; // Add one for the additional space.
						} else {
							for _ in 0..60 - current_line_length {
								html.push(' ');
							}
							html.push_str("<br/>* ");
							html.push_str(word);
							current_line_length = word.len();
						}
					}
				}
				_ => unreachable!(),
			}
		}
		for _ in 0..60 - current_line_length {
			html.push(' ');
		}
		html.push_str("</p>");
		html
	}

	fn render_paragraph(&self, elements: Vec<Token>) -> String {
		let mut html = String::new();
		html.push_str("<p>");
		let mut current_line_length = 0;
		html.push('|');
		for element in elements {
			match element {
				Token::Text(text) => {
					for word in text.split_ascii_whitespace() {
						if word.len() + current_line_length <= 60 {
							if current_line_length != 0 {
								html.push(' ');
							}
							html.push_str(word);

							current_line_length += word.len() + (current_line_length != 0) as usize; // Add one for the additional space.
						} else {
							for _ in 0..60 - current_line_length {
								html.push(' ');
							}
							html.push_str("|<br/>|");
							html.push_str(word);
							current_line_length = word.len();
						}
					}
				}
				_ => unreachable!(),
			}
		}
		for _ in 0..60 - current_line_length {
			html.push(' ');
		}
		html.push_str("|</p>");
		html
	}

	pub fn get_html(&self) -> String {
		let mut html = String::new();
		html.push_str("<!doctype html><head>");
		html.push_str("<style>");
		html.push_str(self.generate_meta().as_str());
		html.push_str("</style>");
		html.push_str("</head>");
		html.push_str("<body><pre>");
		html.push_str(
			"<span aria-label='conner.bond'>
 _______  ___  ___  ___ ____  / /  ___  ___  ___/ /
/ __/ _ \\/ _ \\/ _ \\/ -_) __/ / _ \\/ _ \\/ _ \\/ _  /
\\__/\\___/_//_/_//_/\\__/_/ (_)_.__/\\___/_//_/\\_,_/
</span>",
		);
		for token in MDParser::new(self.text) {
			let render = match token {
				Token::Heading(size, text) => Self::render_heading(size, text),
				Token::Paragraph(elements) => self.render_paragraph(elements),
				Token::Blockquote(elements) => Self::render_blockquote(elements),
				Token::Image { src } => todo!(),
				Token::Codeblock { lang, text } => todo!(),
				// Should not be reachable if parsing is correct
				_ => unreachable!("This tag should not be top level! {:?}", token),
			};
			html.push_str(render.as_str())
		}
		html.push_str("</pre></body>");
		html
	}
}
