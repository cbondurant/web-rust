

//TODO: Remove once out of early prototyping
#![allow(dead_code)]
mod mdparser;

fn main() {

	let mut parser = mdparser::MDParser::parse("This is a test paragraph!\n *and* a continuation\n\nSecond Paragraph\n");

	println!("{:?}", parser.next());
	println!("{:?}", parser.next());
}
