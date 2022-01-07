pub fn abbreviate(phrase: &str) -> String {
    let mut abbr = String::new();

    let phrase_split: Vec<&str> = phrase.split(split_acronym).collect();

    for i in phrase_split {
        if i.is_empty() { continue };
        if is_all_uppercase(i) {
            abbr.push(i.chars().next().unwrap());
        }
        else if len_uppercase_char(i) > 1 {
            for j in i.chars() {
                if j.is_ascii_uppercase() {
                    abbr.push(j);
                }
            }
        }
        else {
            abbr.push(i.chars().next().unwrap().to_ascii_uppercase());
        }
    }
    abbr
}

fn is_all_uppercase(word: &str) -> bool {
    word.chars().all(|c: char| c.is_ascii_uppercase())
}

fn split_acronym(c: char) -> bool {
    if c.is_ascii_whitespace() {
        true
    } else if c == '_' || c == '-' || c == ':' {
        true
    } else {
        false
    }
}

fn len_uppercase_char(word: &str) -> usize {
    word.chars().filter(|c| c.is_ascii_uppercase()).count() as usize
}
