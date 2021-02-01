#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    pins: u16,
    rolls_left: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { score: 0, pins: 10, rolls_left: 20 }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if !(0..=10).contains(&pins) {
            return Err(Error::NotEnoughPinsLeft)
        } else if self.rolls_left == 0 {
            return Err(Error::GameComplete)
        }
        // TODO: implement logic for closed frames (strikes + spares)
        self.rolls_left -= 1;
        self.score += pins;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        
        if self.rolls_left > 0 { None } else { Some(self.score) }
    }
}
