pub fn reply(message: &str) -> &str {
    let message_type = determine_message_type(message);
    
    match message_type {
        "yellquestion" => "Calm down, I know what I'm doing!",
        "question"     => "Sure.",
        "yell"         => "Whoa, chill out!",
        "nothing"      => "Fine. Be that way!",
        "other"        => "Whatever.",
        _              => "Whatever.",
    }
}

fn determine_message_type(msg: &str) -> &str {
    let msg = trim_message(msg);

    let ends_with_question_mark = msg.ends_with('?');
    let is_all_uppercase = msg == msg.to_uppercase();
    let is_empty = msg.is_empty();
    let is_alphabetic = msg.chars().any(|c| c.is_ascii_alphabetic());

    let conditions = (ends_with_question_mark,
        is_all_uppercase, 
        is_alphabetic,
        is_empty);

    match conditions {
        (false, true, false, true) => "nothing",
        (true, true, true, false)  => "yellquestion",
        (true, false, true, false) => "question",
        (false, true, true, false) => "yell",
        _                    => {
            println!("{:?}", msg);
            "other"
        }
    }
}

fn trim_message(msg: &str) -> &str {
    msg.trim_matches(|c: char| {
        c.is_whitespace() || c.is_numeric()
    })
}
