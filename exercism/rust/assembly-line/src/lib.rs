const UNIT_CARS_PRODUCED: f64 = 221.0;
fn get_error_rate(speed: u8) -> f64 {
    if (1..5).contains(&speed) {
        return 1.0;
    }
    else if (5..9).contains(&speed) {
        return 0.9;
    }

    return 0.77;
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    speed  as f64 * UNIT_CARS_PRODUCED * get_error_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60_f64) as u32
}
