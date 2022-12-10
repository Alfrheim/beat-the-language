#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod xml_parser;

use xml_parser::parse_xml_file;
use std::collections::HashMap;
use rand::{thread_rng, seq::SliceRandom};
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
    // DICTIONARY.
        println!("total keys: {}", DICTIONARY.keys().count());//english
        println!("contains empty: {}", DICTIONARY.contains_key(""));//spanish
        // println!("t: {}", t);//definition
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
    let translated_word = get_translation(key);

    println!("----------------");
    println!("selected key: {}", key);
    println!("selected translation: {}", translated_word);
    match language {
        "EN" => CustomResponse {
            word: String::from(key),
            translation: translated_word.clone(),
            choices: vec![translated_word.clone(), get_random_translation(), get_random_translation()]
        },
        _ => CustomResponse { word: "".to_string(), translation: "".to_string(), choices: vec![]}
    }
}

fn get_random_translation() -> String {
    let mut rng = thread_rng();
    // let mut keys: Vec<&String> = DICTIONARY.keys().collect();
    // keys.shuffle(&mut rng);
    loop {
        let key: &String = DICTIONARY.keys().choose(&mut rng).unwrap();
        let result = get_translation(key);
        if !result.trim().is_empty() {
            println!("selected word: {}", result);
            return result.to_string();
        }
    }
}

fn get_translation(key: &String) -> String {
    let (translated_word, _) = DICTIONARY.get(key).unwrap();
    let cleaned_word = translated_word.to_string()
        .replace("{m}", "")
        .replace("{f}", "")
        .replace("{p}", "")
        .split(',')
        .next()
        .unwrap()
        .to_string();
    cleaned_word
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