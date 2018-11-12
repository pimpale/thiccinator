
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::Read;

fn main() {
    let stdin = io::stdin();

    // default rules
    let mut rules: HashMap<String, String> = [
        ("a", "卂"),
        ("b", "乃"),
        ("c", "匚"),
        ("d", "刀"),
        ("e", "乇"),
        ("f", "下"),
        ("g", "厶"),
        ("h", "卄"),
        ("i", "工"),
        ("j", "丁"),
        ("k", "长"),
        ("l", "乚"),
        ("m", "从"),
        ("n", "𠘨"),
        ("o", "口"),
        ("p", "尸"),
        ("q", "㔿"),
        ("r", "尺"),
        ("s", "丂"),
        ("t", "丅"),
        ("u", "凵"),
        ("v", "リ"),
        ("w", "山"),
        ("x", "乂"),
        ("y", "丫"),
        ("z", "乙"),
    ].iter()
        .cloned()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();
	
    // go get rule file if there is one
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        rules = HashMap::new(); // clear it out

        let rule_file = &args[1]; // get rule file name from args

        let mut f = File::open(rule_file).expect("Rule file not found!"); // open file
        let mut contents = "".to_string();
        f.read_to_string(&mut contents).expect(
            "Could not read file!",
        ); // write to file

        // parse file
        let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
        for line in lines {
            let stuff: Vec<String> = line.split(char::is_whitespace)
                .map(|s| s.to_string())
                .collect();
            if stuff.len() > 1 {
                rules.insert(stuff[0].clone(), stuff[1].clone());
            }
        }
    }

    for nl in stdin.lock().lines().filter_map(|l| {
        l.map(|l: String| rule_replace(l.to_string(), &rules)).ok()
    })
    {
        println!("{}", nl);
    }
}

fn rule_replace(text: String, hash: &HashMap<String, String>) -> String {
    hash.iter().fold(text.to_lowercase(), |txt, subst| {
        str::replace(txt.as_str(), subst.0, subst.1)
    })
}
