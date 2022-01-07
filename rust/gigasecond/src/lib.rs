// Bring Duration struct into scope
use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // "seconds" is an impl of Duration struct
    start + Duration::seconds(1_000_000_000)
}
