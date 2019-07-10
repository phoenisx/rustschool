use chrono::{DateTime, Duration, Utc};
use std::ops::Add;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
  let ten_to_nine = Duration::seconds(10i64.pow(9));
  start.add(ten_to_nine)
}
