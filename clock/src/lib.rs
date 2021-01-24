use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours_result, minutes_result) = convert_hours_and_minutes(
            hours, 
            minutes,
        );
        Clock {
            hours: hours_result,
            minutes: minutes_result,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours_result, minutes_result) = convert_hours_and_minutes(
            self.hours, 
            self.minutes + minutes,
        );
        
        Clock {
            hours: hours_result,
            minutes: minutes_result,
        }
    }
}

fn convert_hours_and_minutes(hours: i32, minutes: i32) -> (i32, i32) {
    let (mut hours_result, mut minutes_result) = (hours, minutes);

    // convert minutes to 0 - 59 values, adjust hours accordingly
    loop {
        if minutes_result > 59 {
            minutes_result -= 60;
            hours_result += 1;
        } else if minutes_result < 0 {
            minutes_result += 60;
            hours_result -= 1;
        } else {
            break;
        }
    }

    // correct hours to values between 0 and 24
    loop {
        if hours_result > 23 {
            hours_result -= 24;
        } else if hours_result < 0 {
            hours_result += 24;
        } else {
            break;
        }
    }

    (hours_result, minutes_result)
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}