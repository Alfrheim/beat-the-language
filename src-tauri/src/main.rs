#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod translate;
mod xml_parse_serde;

use crate::{
    translate::get_verbs,
    xml_parse_serde::{get_list_of_words, Word, Words},
};
use rand::{seq::SliceRandom, thread_rng};
extern crate quick_xml;

lazy_static! {
    static ref WORDS: Words = get_list_of_words();
}
lazy_static! {
    static ref VERBS: Words = get_verbs("src/resources/spanish_verbs_clean.txt");
}

fn main() {
    println!("document total words: {}", WORDS.count()); //english
    println!("document total verbs: {}", VERBS.count()); //english
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_word, get_verb])
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
fn get_word(language: &str) -> CustomResponse {
    let word = WORDS.random_word();
    match language {
        "EN" => {
            let mut choices = vec![
                clean_translation(word),
                clean_translation(WORDS.random_word()),
                clean_translation(WORDS.random_word()),
            ];
            choices.shuffle(&mut thread_rng());
            let response = CustomResponse {
                word: word.word.to_string(),
                translation: clean_translation(word),
                choices,
            };
            response
        }
        _ => CustomResponse {
            word: "".to_string(),
            translation: "".to_string(),
            choices: vec![],
        },
    }
}

#[tauri::command]
fn get_verb(language: &str) -> CustomResponse {
    let word = VERBS.random_word();
    match language {
        "EN" => {
            let mut choices = vec![
                word.translation.to_string(),
                VERBS.random_word().translation.to_string(),
                VERBS.random_word().translation.to_string(),
            ];
            choices.shuffle(&mut thread_rng());
            let response = CustomResponse {
                word: word.word.to_string(),
                translation: word.translation.to_string(),
                choices,
            };
            response
        }
        _ => CustomResponse {
            word: "".to_string(),
            translation: "".to_string(),
            choices: vec![],
        },
    }
}

fn clean_translation(word: &Word) -> String {
    let cleaned_word = word
        .translation
        .to_string()
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
    use crate::get_word;

    #[test]
    fn it_works() {
        get_word("EN");
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
