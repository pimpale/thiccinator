use std::io::{self, BufRead};
fn main() {
	let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}",transform(line.unwrap()));
    }
}

fn transform(text:String) -> String
{
	let mut transformed = String::new();
	for c in text.to_lowercase().chars() {
            transformed = format!("{}{}", transformed, regionate(c));
	}
	transformed 
}

fn regionate(character:char) -> String {
    format!(":regional_indicator_{}: ", character)
}

fn thiccinate(character:char) -> String {
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
		_ => character
		}.to_string()
}
