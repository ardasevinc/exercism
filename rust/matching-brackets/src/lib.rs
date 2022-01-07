// All credits to user michaelmez39
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_string: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '(' | '[' | '{' => bracket_string.push(c),
            ')' => if bracket_string.pop() != Some('(') { return false },
            ']' => if bracket_string.pop() != Some('[') { return false },
            '}' => if bracket_string.pop() != Some('{') { return false },
             _  => ()
        }
    }
    bracket_string.is_empty()
}