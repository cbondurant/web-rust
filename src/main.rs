

//TODO: Remove once out of early prototyping
#![allow(dead_code)]
mod mdparser;

fn main() {

	let parser = mdparser::MDParser::parse(
"# This is my level 1 header
## This is my level 2 header

Wow!!!
This is a test paragraph!
*and* a continuation

Second Paragraph");

	for token in parser{
		println!("{:?}", token);
	}
}
