extern crate xml;

use std::collections::HashMap;

 use std::fs::File;
 use std::io::Read;

 use self::xml::{Event, Parser};

#[derive(Debug)]
pub struct Config{
	config : HashMap<String , String>
}

impl Config
{
	pub fn new() -> Config {
		Config {
			config : HashMap::new()
		}
	}

	pub fn init(mut self, file_name : & str) ->Self{

		let mut rdr = match File::open(file_name) {
			Ok(file) => file,
			Err(err) => {
				panic!("Couldn't open file: {}", err);
			}
		};

		let mut p = Parser::new();

		let mut string = String::new();
		if let Err(err) = rdr.read_to_string(&mut string) {
			panic!("Reading failed: {}", err);
		};

		let mut content = String::new();
		p.feed_str(&string);
		for event in p {
			match event.unwrap() {

				Event::ElementEnd(tag) => {
							if !content.trim().is_empty(){
								self.config.insert(tag.name,content.clone());
							}
						} ,
				Event::Characters(data) => content = data,
				_ => ()
			}
		}
		self
	}

	pub fn get(& self , key : &str) -> Option<&String>{
        self.config.get(key)
	}
}
