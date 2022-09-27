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
>
> Blockquote pt.iii

> Second Blockquote thai is extremely long and doing all the work that I need in order to type all the shit i can

Wow!!!
This is a test paragraph!
*and* a continuation that [https://google.com](needs) to be a lot longer to ensure the length of all of this stuff.
we wanna make sure that our linebreaking everything works without issue.

Second Paragraph",
	);

	println!("{}", parser.get_html());
}
