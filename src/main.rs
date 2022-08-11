use std::{
    env,  
    path::Path,
    io::prelude::*,
    fs::{
        OpenOptions, 
        read_to_string
    }
};
use lazy_static::lazy_static;
use regex::Regex;

fn normalize_spaces(text: &str) -> String {

    lazy_static! {
        // Pattern to detect / work with:
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }

    // Replace detected pattern `\s+` within `&text`, with `" "`, a single empty space.
    // Convert pointer `Cow<&str>` into `String`
    RE.replace_all(&text, " ").into()
}

fn main() {

    let txt_file = env::args().nth(1).expect("Missing first argument for input .txt file.");

    let txt_file = Path::new(&txt_file);
    
    if txt_file.is_file() {

        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s+").unwrap();
        }

        let data = read_to_string(txt_file).expect("Unable to open .txt file");

        let output_data = data.lines().map(|line|format!("{}\r\n", normalize_spaces(line))).collect::<String>();

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(txt_file)
            .expect("Unable to open .txt file");

        file
            .write(output_data.as_bytes())
            .expect("Unable to update .txt file");
         
        
    } else {
        eprintln!("'{}' file doesn't exist.", txt_file.display());
    }
}
