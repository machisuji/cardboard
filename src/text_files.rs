use std::io::prelude::*;
use std::fs::File;

use std::io::Result;

pub fn update_text_file<F>(file_name: &str, mut f: F) -> Result<()> where F: FnMut(String, &mut String) {
    let mut file = try!(File::open(file_name));
    let mut input = String::new();
    let mut output = String::new();

    if file.read_to_string(&mut input).is_ok() {
        f(input, &mut output);

        let mut dst = try!(File::create(file_name));

        try!(dst.write_all(output.as_bytes()));
    }

    Ok(())
}

pub fn create_text_file(file_name: &str, content: String) -> Result<()> {
    let mut dst = try!(File::create(file_name));

    try!(dst.write_all(content.as_bytes()));

    Ok(())
}
