//TODO: Remove once out of early prototyping
#![allow(dead_code)]
mod mdparser;
mod pagerender;

use pagerender::PageRenderer;

fn main() {
	let parser = PageRenderer::new(
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

	println!("{}", parser.get_html());
}
