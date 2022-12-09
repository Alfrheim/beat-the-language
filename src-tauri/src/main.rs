#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod xml_parser;

use xml_parser::parse_xml_file;
use std::collections::HashMap;
use rand::{thread_rng};
use rand::seq::IteratorRandom;
use serde_json::to_string;

extern crate quick_xml;

lazy_static! {
    static ref DICTIONARY: HashMap<String, (String, String)> = {
        parse_xml_file()
    };
}

fn main() {
      // Open and read the XML file

    // Parse the XML file and extract its content
    // let extracted_content = parse_xml_file();

    // Print the extracted content
    // for (c, (d, t)) in DICTIONARY.iter() {
    //     println!("c: {}", c);//english
    //     println!("d: {}", d);//spanish
    //     println!("t: {}", t);//definition
    // }
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![get_random_word])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(serde::Serialize)]
struct CustomResponse {
    word: String,
    translation: String,
    choices: Vec<String>,
}

#[tauri::command]
fn get_random_word(language: &str) -> CustomResponse {

    let mut rng = thread_rng();
    let key: &String = DICTIONARY.keys().choose(&mut rng).unwrap();

    // let key = seq::sample_iter(&mut rng, DICTIONARY.keys(),1).unwrap();
    let (translated_word, _) = DICTIONARY.get(key).unwrap();
    let cleaned_word = translated_word.to_string()
        .replace("{m}", "")
        .replace("{f}", "")
        .split(',')
        .next()
        .unwrap()
        .to_string();

    match language {
        "EN" => CustomResponse {
            word: String::from(key),
            translation: cleaned_word.clone(),
            choices: vec![cleaned_word.clone(), "Champu".to_string(), "Radiador".to_string()]
        },
        _ => CustomResponse { word: "".to_string(), translation: "".to_string(), choices: vec![]}
    }
}

#[cfg(test)]
mod tests {
    use crate::get_random_word;

    #[test]
    fn it_works(){
        get_random_word("EN");
        let result = 2+2;
        assert_eq!(result, 4);
    }
}