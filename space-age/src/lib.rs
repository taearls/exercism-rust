#[derive(Debug)]
pub struct Duration {
    len: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let earth_seconds = 31557600.0;
        Duration {
            len: s as f64 / earth_seconds,
        }
    }
}

pub trait Planet {
    fn orbital_period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.len / Self::orbital_period()
    }
}

macro_rules! planet {
    ($planet_name:ident, $orbital_period:expr) => {
        pub struct $planet_name;
        impl Planet for $planet_name {
            fn orbital_period() -> f64 {
                $orbital_period
            }
        }
    };
}
planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
