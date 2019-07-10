use chrono::{DateTime, Duration, Utc};
use std::ops::Add;
use std::time;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
  let ten_to_nine = Duration::from_std(time::Duration::new(10u64.pow(9), 0))
    .ok()
    .unwrap();
  start.add(ten_to_nine)
}
