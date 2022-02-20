pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base_hourly_rate: f64 = 221. * (speed as f64);
    let success_rate: f64 = match speed {
        0 => 0.,
        1..=4 => 1.,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => panic!("Invalid speed provided: {}", speed),
    };
    base_hourly_rate * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.) as u32
}
