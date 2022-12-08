#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod xml_parser;

use std::borrow::Borrow;
use xml_parser::parse_xml_file;
use std::collections::HashMap;
extern crate quick_xml;

lazy_static! {
    static ref DICTIONARY: HashMap<String, (String, String)> = {
        parse_xml_file()
    };
}

fn main() {
      // Open and read the XML file

    // Parse the XML file and extract its content
    let extracted_content = parse_xml_file();

    // Print the extracted content
    for (c, (d, t)) in DICTIONARY.iter() {
        println!("c: {}", c);
        println!("d: {}", d);
        println!("t: {}", t);
    }
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![get_random_word])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_random_word(language: &str) -> String {

    let extracted_content = parse_xml_file();
    match language {
        "EN" => String::from("House"),
        _ => String::from("couldn't recover the word")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works(){
        let result = 2+2;
        assert_eq!(result, 4);
    }
}