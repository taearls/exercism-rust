use chrono::{DateTime, Utc, NaiveDateTime};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // convert DateTime<Utc> to timestamp, add 1 billion seconds
    let start_timestamp = DateTime::timestamp(&start) + 1_000_000_000;
    
    // convert from new timestamp back to DateTime<Utc>
    let end = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(start_timestamp, 0), Utc);
    end
}
