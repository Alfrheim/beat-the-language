use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::xml_parse_serde::{Word, Words};

pub(crate) fn get_verbs(filename: &str) -> Words {
    let mut result = Vec::new();
    //let filename = &"src/resources/spanish_verbs_clean.txt";
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            result.push(to_word(line.unwrap()));
        });
    }
    Words::new(result)
}

fn to_word(line: String) -> Word {
    let mut split = line.split('|');
    let verb = split.next();
    let translation = split.next();
    Word {
        word: verb.unwrap().trim().to_string(),
        translation: translation
            .unwrap_or_else(|| {
                println!("{}", verb.unwrap());
                return "";
            })
            .to_string(),
        definition: "".to_string(),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_parse_all_verbs() {
        get_verbs("src/resources/spanish_verbs_clean.txt");
    }

    #[test]
    fn extract_the_verb_into_the_word() {
        let verbs = get_verbs("src/resources/test/spanish_verbs.txt");
        let result = verbs.random_word();
        assert_eq!(result.word, "acentuar");
    }
    #[test]
    fn extract_the_translation_into_the_word() {
        let verbs = get_verbs("src/resources/test/spanish_verbs.txt");
        let result = verbs.random_word();
        assert!(result
            .translation
            .contains("to accentuate, mark with an accent"));
    }
}
