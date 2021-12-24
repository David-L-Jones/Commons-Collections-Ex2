use std::io;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("\n*** Word to Pig Latin ***\n");

    loop {
        println!("Enter a word or Q to quit");
        let mut word = String::new();

        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        // Trim the new line character
        let word = word.trim();

        if word.to_uppercase() == "Q" {
            println!("Goodbye");
            break;
        } else {
            println!("{}", convert_to_pig_latin(word));
        }
    }
}

/// Convert strings to pig latin. The first consonant of each word is
/// moved to the end of the word and “ay” is added, so “first”
/// becomes “irst-fay.” Words that start with a vowel have “hay” added
/// to the end instead (“apple” becomes “apple-hay”).
fn convert_to_pig_latin(word: &str) -> String {
    let first_grapheme = word.graphemes(true).next().unwrap();
    let vowels = vec!["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];

    if vowels.contains(&first_grapheme) {
        format!("{}-hay", word)
    } else {
        let g = word.graphemes(true).collect::<Vec<&str>>();
        let mut i = 1;
        let mut gstring = String::new();

        while i < g.len() {
            match g.get(i) {
                Some(w) => gstring.push_str(w),
                None => (),
            }

            i += 1;
        }
        
        format!("{}-{}ay", gstring, &first_grapheme)
    }
}
