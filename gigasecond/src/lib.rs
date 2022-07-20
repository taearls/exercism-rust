use time::{PrimitiveDateTime, Duration};

// Returns a PrimitiveDateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    start.saturating_add(Duration::seconds(1_000_000_000))
}
