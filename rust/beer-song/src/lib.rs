pub fn verse(n: u32) -> String {
    let mut final_verse = String::new();

    match n {
        0 => {
            final_verse = "No more bottles of beer on the wall, no more bottles of beer.\
            \nGo to the store and buy some more, \
            99 bottles of beer on the wall.\n".to_string();
        },
        
        1 => {
            final_verse = "1 bottle of beer on the wall, 1 bottle of beer.\
             \nTake it down and pass it around, \
             no more bottles of beer on the wall.\n".to_string();
        },

        2 => {
            final_verse ="2 bottles of beer on the wall, 2 bottles of beer.\
             \nTake one down and pass it around, \
             1 bottle of beer on the wall.\n".to_string();
        },

        _ => {
            final_verse = format!("{0} bottles of beer on the wall, {0} bottles of beer.\
            \nTake one down and pass it around, \
            {1} bottles of beer on the wall.\n", n, n-1);
        },
    }
    final_verse
}

pub fn sing(start: u32, end: u32) -> String {
    let mut final_song = String::new();

    for i in (end..=start).rev() {
        // Solves the big introduced by the unwanted newline-
        // that occurs when i == last verse
        if (end != start) && i != end {
            final_song.push_str(&verse(i));
            final_song.push_str("\n");
            
        } else {
            final_song.push_str(&verse(i));
        }

    }
    final_song
}
