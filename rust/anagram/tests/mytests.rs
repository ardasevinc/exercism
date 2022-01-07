use std::collections::HashSet;

fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
    let result = anagram::anagrams_for(word, inputs);

    let expected: HashSet<&str> = expected.iter().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_word_capitalization() {
    let word = "TesT";
    
    let inputs = ["poop"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}
