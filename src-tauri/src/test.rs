
struct Word {
    word: String,
    definition: String,
    translation: String,
}

#[test]
fn check_we_parse_the_file() {
    let expected = Word {
        word: String::from("casa"),
        definition: String::from("where people live"),
        translation: String::from("casa")
    };
}