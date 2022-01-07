use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Rust's % operator is std::ops::Rem, remainder, not modulus.
        // It was very painful to realize this for me
        // rem_euclid method on i32 can be used instead
        // div_euclid is similar to `//` operator in python
        
        // Find how many hours is in the given minute and add hours
        // do this all in mod 24
        let hours = (minutes.div_euclid(60) + hours).rem_euclid(24);
        let minutes = minutes.rem_euclid(60);
        Clock {
            hours,
            minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Figured it would be simpler to convert everything to minutes,
        // do the calculation, then convert back to Clock
        let old_total_minutes: i32 = (self.hours * 60) + self.minutes;
        let new_total_minutes = old_total_minutes + minutes;

        // First find hor many hours is in the new total minutes
        // then convert to mod 24
        let new_hours = (new_total_minutes.div_euclid(60)).rem_euclid(24);
        let new_minutes = new_total_minutes.rem_euclid(60);
        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // width of 2, pad with zeros
        // more info on std documentation `std::fmt`
        write!(f, "{h:0>2}:{m:0>2}", h=self.hours, m=self.minutes)
    }
}
