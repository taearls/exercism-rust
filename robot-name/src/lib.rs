const ROBOT_NAME_LEN: usize = 5;
const LETTER_COUNT: usize = 2;

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = gen_robot_name(&self.name);
    }
}

impl Default for Robot {
    fn default() -> Self {
        Robot {
            name: gen_robot_name(""),
        }
    }
}

fn gen_robot_name(old_name: &str) -> String {
    let mut name = String::with_capacity(ROBOT_NAME_LEN);

    for i in 0..ROBOT_NAME_LEN {
        let modulo: u8;
        let ascii_offset: u8;

        if i < LETTER_COUNT {
            modulo = 26;
            ascii_offset = 65; // 65 = 'A'
        } else {
            modulo = 10;
            ascii_offset = 48; // 48 = '0'
        }
        let c = (rand::random::<u8>() % modulo + ascii_offset) as char;
        name.push(c);
    }

    if name == old_name {
        name = gen_robot_name(old_name);
    }
    name
}
