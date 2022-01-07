use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;
/* 
    UniCase is a crate that creates a wrapper for strings
    which allows unicode case insensitive string comparisons 
    using case-folding
*/
use unicase::UniCase;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    let word_processed = process(word);

    for candidate in possible_anagrams {
        if candidate.len() != word.len() { continue }

        let candidate_processed = process(candidate);

        let is_eq = candidate_processed == word_processed;
        let is_same_str = UniCase::new(candidate) == UniCase::new(word);

        if is_eq && !is_same_str {
            anagrams.insert(*candidate);
        }
    }
    anagrams
}

fn process(thing: &str) -> UniCase<String> {
    let thing_graphemes: Vec<&str> = thing
        .graphemes(true)
        .collect();

    // Wrap each grapheme with UniCase
    let mut thing_unicase: Vec<UniCase<&str>> = thing_graphemes
        .into_iter()
        .map(|x| UniCase::new(x))
        .collect();

    /*
        Since UniCase uses cases folding and implements Ord and Eq
        we can just sort this and sort_unstable will use those traits
        that UniCase implements
        And voila, sorting with no case changes!
    */
    thing_unicase.sort_unstable();

    /*
    // It wouldn't let me collect into a UniCase<String>
    // Something about trait bounds
    // So i manually collected it
    */
    let mut s = String::new();

    thing_unicase.iter().for_each(|x| s.push_str(x));
    /*
        We still return a UniCase string 
        it's needed in the final comparison
    */
    UniCase::new(s)
}
