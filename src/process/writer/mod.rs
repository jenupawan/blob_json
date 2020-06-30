use std::fs::File;
use std::io::Write;

pub fn write_to_file(mut output_file: &File, jsonstring:&str) {
  output_file
      .write_all(jsonstring.as_bytes())
      .expect("unable to write to output file");
}
