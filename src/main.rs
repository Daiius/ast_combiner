
extern crate clap;
use clap::{App, Arg};

use std::path::Path;

use std::fs::File;

use std::io::{Read, Write};

fn main() {
    let matches = App::new("ASCII STL (.ast) file combiner")
		.version("0.1")
		.author("Daiji Yamashita <yamashita-daiji@sei.co.jp")
		.about("Combines ASCII STL (.ast) file combiner")
		.arg(Arg::with_name("INPUTS")
			.short("i")
			.long("input")
			.help("Input files to combine (.ast)")
			.required(true)
			.takes_value(true)
			.multiple(true))
		.arg(Arg::with_name("OUTPUT")
			.short("o")
			.long("output")
			.help("Output file (ascii .stl)")
			.required(true)
			.takes_value(true))
		.get_matches();

	let input_file_names: Vec<_> = matches.values_of("INPUTS").unwrap().collect();

	let mut buffer = String::new();
	for input_file_name in input_file_names {
		let mut tmp_buffer = String::new();
		
		// Read file to buffer
		let mut file = File::open(input_file_name)
			.expect(&format!("Failed to load input file {}", input_file_name));
		file.read_to_string(&mut tmp_buffer)
			.expect(&format!("File read failed...: {}", input_file_name));

		// Check solid or endsolid count: should be 1.
		let solid_count = tmp_buffer.rmatches("endsolid").count();
		if solid_count != 1 {
			println!("Error: solid count in a .ast file is expected to be only 1.");
			println!("-> there are {} solids in a file, {}", solid_count, input_file_name);
			return;
		}
		
		let path = Path::new(input_file_name);
		let model_name = path.file_stem().unwrap().to_str().unwrap();

		// Replacing "Mesh" into file name stem
		tmp_buffer = tmp_buffer.replace("Mesh", model_name);
		
		buffer.push_str(&tmp_buffer);
	}

	let output_file_name = matches.value_of("OUTPUT").unwrap();
	let mut output_file = File::create(output_file_name)
		.expect(&format!("Failed to create output file: {}", output_file_name));

	output_file.write_all(buffer.as_bytes()).expect(&format!("Failed to write output file, {}", output_file_name));

	println!("Output file: {} was created, Succeed!", output_file_name);


}
