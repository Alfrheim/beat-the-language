use quick_xml::de::{from_str, DeError};
use serde::Deserialize;
use std::fs;

extern crate rand;
use rand::prelude::*;

#[derive(Debug, Deserialize, Default)]
struct Dic {
    l: Vec<L>, // groups by first letter (a, b, c, ...)
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct L {
    #[serde(rename = "w")]
    words: Vec<Word>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub(crate) struct Word {
    #[serde(rename = "c")]
    pub(crate) word: String,
    #[serde(rename = "d")]
    pub(crate) translation: String,
    #[serde(rename = "t")]
    pub(crate) definition: String,
}

pub(crate) struct Words {
    words: Vec<Word>,
}

impl Words {
    pub fn new(words: Vec<Word>) -> Self {
        Words { words }
    }
    pub fn count(&self) -> usize {
        self.words.len()
    }

    pub fn random_word(&self) -> &Word {
        self.words.choose(&mut rand::thread_rng()).unwrap()
    }
}

fn parse_xml_serde_file() -> Result<Dic, DeError> {
    let file = fs::read_to_string("src/resources/en-es.xml").unwrap();
    let content: &str = &file;

    // Create a new XML reader
    let doc: Dic = from_str(content)?;
    Ok(doc)
}

pub(crate) fn get_list_of_words() -> Words {
    let dictionary = parse_xml_serde_file().unwrap();
    let words = dictionary
        .l
        .iter()
        .flat_map(|l| l.words.iter())
        .cloned()
        .collect();
    Words::new(words)
}

#[cfg(test)]
mod tests {
    use crate::xml_parse_serde::Dic;
    use quick_xml::de::from_str;

    #[test]
    fn parses_the_document() {
        let dictionary_xml: &str = r###"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Dic from="en" to="es">
    <l>
                <w>
            <c>house</c>
            <d>casa {f}</d>
            <t>{n} /eɪ/ (name of the letter A, a)</t>
        </w>
        <w>
            <c>Aachen</c>
            <d>Aquisgrán</d>
            <t>{prop} /ˈɑ.kən/ (city in North Rhine-Westphalia)</t>
        </w>
        </l>
        </Dic>
        "###;

        let result: Dic = from_str(dictionary_xml).unwrap();
        assert_eq!(result.l.get(0).unwrap().words.len(), 2);
        assert_eq!(
            result.l.get(0).unwrap().words.get(0).unwrap().word,
            "house".to_string()
        );
    }
}
