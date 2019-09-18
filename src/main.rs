use std::io::{self, BufRead};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 || args[1] == "--help" {
        println!(
            "Usage: thiccinator [OPTION]
Transform text read from standard input

--help                view this usage
--thiccinate          convert text to pseudo-kanji characters
--regionate           converts each character to discord regional indicator"
        );
    } else {
        let fun: &Fn(char) -> String = if args[1] == "--regionate" {
            &regionate
        } else {
            &thiccinate
        };

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            println!("{}", transform(line.unwrap(), fun));
        }
    }
}

fn transform(text: String, fun: &Fn(char) -> String) -> String {
    let mut transformed = String::new();
    for c in text.to_lowercase().chars() {
        transformed = format!("{}{}", transformed, fun(c));
    }
    transformed
}

fn regionate(character: char) -> String {
    if character.is_alphabetic() {
        format!(":regional_indicator_{}:", character.to_lowercase())
    } else {
        character.to_string()
    }
}

fn thiccinate(character: char) -> String {
    match character {
        'a' => '卂',
        'b' => '乃',
        'c' => '匚',
        'd' => '刀',
        'e' => '乇',
        'f' => '下',
        'g' => '厶',
        'h' => '卄',
        'i' => '工',
        'j' => '丁',
        'k' => '长',
        'l' => '乚',
        'm' => '从',
        'n' => '𠘨',
        'o' => '口',
        'p' => '尸',
        'q' => '㔿',
        'r' => '尺',
        's' => '丂',
        't' => '丅',
        'u' => '凵',
        'v' => 'リ',
        'w' => '山',
        'x' => '乂',
        'y' => '丫',
        'z' => '乙',
        _ => character,
    }
    .to_string()
}
