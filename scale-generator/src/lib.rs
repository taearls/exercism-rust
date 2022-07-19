use lazy_static::lazy_static;
use regex::Regex;

// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub enum Error {
    InvalidTonic,
    InvalidIntervals,
}

pub struct Scale {
    tonic: String,
    intervals: String,
}

fn is_valid_intervals_str(intervals_str: &str) -> bool {
    lazy_static! {
        static ref INTERVAL_REGEX: Regex = Regex::new(r"^[mMaA]*$").unwrap();
    }
    INTERVAL_REGEX.is_match(intervals_str)
}

fn is_valid_note_str(note_str: &str) -> bool {
    lazy_static! {
      // check if str has a-g or A-G in one occurrence
      // check for one flat, or one sharp
      static ref NOTE_REGEX: Regex = Regex::new(
        r"^(?P<note_name>(?i)[a-g]{1})(?P<note_variant>(?-i)(b{1})|(#{1}))?$"
      ).unwrap();
    }
    NOTE_REGEX.is_match(note_str)
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        if !is_valid_note_str(tonic) {
            Err(Error::InvalidTonic)
        } else if !is_valid_intervals_str(intervals) {
            Err(Error::InvalidIntervals)
        } else {
            Ok(Scale {
                tonic: tonic.to_string(),
                intervals: intervals.to_string(),
            })
        }
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        if !is_valid_note_str(tonic) {
            Err(Error::InvalidTonic)
        } else {
            Ok(Scale {
                tonic: tonic.to_string(),
                intervals: "m".repeat(11).to_string(),
            })
        }
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut notes = vec![self.tonic.clone()];
        for interval in self.intervals.chars() {
            match interval {
                'm' => notes.push(notes.last().unwrap().clone()),
                'M' => notes.push(notes.last().unwrap().clone()),
                'A' => notes.push(notes.last().unwrap().clone()),
                'a' => notes.push(notes.last().unwrap().clone()),
                _ => panic!("Invalid interval"),
            }
        }
        notes
    }
}
