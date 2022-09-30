//TODO: Remove once out of early prototyping
#![allow(dead_code)]
mod mdparser;
mod pagerender;

use pagerender::PageRenderer;

use std::{fs::File, io::Read};

fn main() {
	let mut config_file = match File::open("wust.toml") {
		Err(why) => panic!("Could not open configuration file wust.toml: {}", why),
		Ok(file) => file,
	};

	let mut data = String::new();
	match config_file.read_to_string(&mut data) {
		Err(why) => panic!("Failed to read from configuration file wust.toml: {}", why),
		Ok(_) => (),
	}
	let config = data.parse::<toml::Value>().unwrap();

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
*and* a continuation that [needs](https://google.com) to be a lot longer to ensure the length of all of this stuff.
we wanna make sure that our linebreaking everything works without issue.

Second Paragraph",
		config
	);

	println!("{}", parser.get_html());
}
