#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
pub enum BowlingFrameState {
    Open,
    Spare,
    Strike,
    Complete,
}

#[derive(Debug)]
pub struct BowlingFrame {
    score: u16,
    rolls: u8,
    state: BowlingFrameState,
    pins_left: u16,
}

pub struct BowlingGame {
    frames: Vec<BowlingFrame>,
    is_game_over: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: vec![],
            is_game_over: false,
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_over {
            None
        } else {
            let mut score = 0;
            for frame in self.frames.iter() {
                score += frame.score;
            }
            Some(score)
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if !(0..=10).contains(&pins) {
            return Err(Error::NotEnoughPinsLeft);
        } else if self.is_game_over {
            return Err(Error::GameComplete);
        }

        match self.frames.len() {
            0 => {
                let state = match pins {
                    10 => BowlingFrameState::Strike,
                    _ => BowlingFrameState::Open,
                };
                let frame = BowlingFrame {
                    state,
                    score: pins,
                    rolls: 1,
                    pins_left: (10 - pins),
                };
                self.frames.push(frame);
            }
            len @ 1..=10 => {
                if let Err(Error::NotEnoughPinsLeft) = BowlingGame::normal_score(self, pins) {
                    return Err(Error::NotEnoughPinsLeft);
                }
                BowlingGame::check_spare(self, pins);
                BowlingGame::check_strike(self, pins);
                if len == 10 && self.frames.last().unwrap().state == BowlingFrameState::Complete
                    || self.frames.last().unwrap().rolls == 3
                {
                    self.is_game_over = true;
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn normal_score(&mut self, pins: u16) -> Result<(), Error> {
        // add a new frame if previous frame is closed or completed
        BowlingGame::check_add_frame(self);

        let last_frame_index = self.frames.len() - 1;
        let last_frame = self.frames.get_mut(last_frame_index).unwrap();

        if pins > last_frame.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }

        last_frame.score += pins;
        last_frame.pins_left -= pins;
        last_frame.rolls += 1;

        if last_frame.pins_left == 0 {
            if last_frame.rolls == 1 {
                last_frame.state = BowlingFrameState::Strike;
            } else if last_frame.rolls == 2 {
                last_frame.state = BowlingFrameState::Spare;
            }
            // in the last frame, we have to reset the pin count if it's a closed frame.
            if last_frame_index + 1 == 10 {
                last_frame.pins_left = 10;
            }
        } else if last_frame.rolls == 2 && last_frame.state == BowlingFrameState::Open {
            last_frame.state = BowlingFrameState::Complete;
        }
        Ok(())
    }

    fn check_spare(&mut self, pins: u16) {
        if self.frames.len() > 1 {
            let previous_frame_index = self.frames.len() - 2;
            let previous_frame = self.frames.get_mut(previous_frame_index).unwrap();
            if previous_frame.state == BowlingFrameState::Spare && previous_frame.rolls < 3 {
                previous_frame.score += pins;
                previous_frame.rolls += 1;
                previous_frame.state = BowlingFrameState::Complete;
            }
        }
    }

    fn check_strike(&mut self, pins: u16) {
        // only check two frames before if the previous frame is a strike
        let mut should_check_two_frames_previous = false;

        if self.frames.len() > 1 {
            let previous_frame_index = self.frames.len() - 2;
            let previous_frame = self.frames.get_mut(previous_frame_index).unwrap();
            if previous_frame.state == BowlingFrameState::Strike && previous_frame.rolls < 3 {
                previous_frame.score += pins;
                previous_frame.rolls += 1;
                // only check two frames before if there are enough frames
                should_check_two_frames_previous = previous_frame_index > 0;
                if previous_frame.rolls == 3 {
                    previous_frame.state = BowlingFrameState::Complete;
                }
            }
        }

        if should_check_two_frames_previous {
            let previous_frame_index = self.frames.len() - 3;
            let previous_frame = self.frames.get_mut(previous_frame_index).unwrap();
            if previous_frame.state == BowlingFrameState::Strike {
                previous_frame.score += pins;
                previous_frame.rolls += 1;
                if previous_frame.rolls == 3 {
                    previous_frame.state = BowlingFrameState::Complete;
                }
            }
        }
    }

    fn check_add_frame(&mut self) {
        if self.frames.len() < 10 && self.frames.last().unwrap().state != BowlingFrameState::Open {
            let frame = BowlingFrame {
                state: BowlingFrameState::Open,
                rolls: 0,
                score: 0,
                pins_left: 10,
            };
            self.frames.push(frame);
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for BowlingFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_string = match self.state {
            BowlingFrameState::Open => "open",
            BowlingFrameState::Complete => "complete",
            BowlingFrameState::Spare => "spare",
            BowlingFrameState::Strike => "strike",
        };
        write!(
            f,
            "score: {}, rolls: {}, state: {}, pins_left: {}",
            self.score, self.rolls, state_string, self.pins_left
        )
    }
}
