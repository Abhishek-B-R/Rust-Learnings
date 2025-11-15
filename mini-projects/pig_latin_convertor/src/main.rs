// Rules:
// - If a word starts with a vowel (a, e, i, o, u), append “way”: apple → appleway
// - If it starts with a consonant cluster, move the leading consonants to the end and append “ay”: string → ingstray
// - Preserve case per word, ignore punctuation for simplicity (optional enhancements below)

use std::io;

fn main() {
    let mut word = String::new();

    println!("Enter your string that has to be converted to pig latin:");
    io::stdin()
        .read_line(&mut word)
        .expect("Couldn't read input");
    
    let first_char : char = word.chars().next().expect("couldn't find a valid string");
    let mut word = word.trim().to_string();

    if check_char_type(first_char){
        word.push_str("way");
    } else {
        let mut new_word = String::new();
        let mut temp = String::new();
        let mut vowel_detected = false;

        for i in word.chars(){
            if check_char_type(i) {
                vowel_detected = true;
                new_word.push(i);
            }else if !vowel_detected{
                temp.push(i);
            }else{
                new_word.push(i);
            }
        }
        if !vowel_detected {
            new_word = word.clone();
        }

        new_word.push_str(&temp);
        new_word.push_str("ay");
        word = new_word;
    }

    println!("Pig latin word - {}", word);
}

fn check_char_type(x:char) -> bool {
    matches!(x.to_ascii_lowercase(), 'a'|'e'|'i'|'o'|'u')
}