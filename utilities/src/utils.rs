use crate::Error;
use chrono::LocalResult::{Ambiguous, Single};
use chrono::{DateTime, MappedLocalTime, TimeZone, Utc};
use nanoid::nanoid;

const ALPHABET: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Generate a new id
pub fn generate_new_id() -> String {
    nanoid!(10, &ALPHABET)
}

pub fn timestamp_to_date_time(timestamp: i64) -> Result<DateTime<Utc>, Error> {
    match Utc.timestamp_opt(timestamp, 0) {
        Single(d) => Ok(d),
        Ambiguous(s, _) => Ok(s),
        MappedLocalTime::None => Err(Error::new("Invalid timestamp value")),
    }
}
