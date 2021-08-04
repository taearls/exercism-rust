const ROBOT_NAME_LEN: usize = 5;

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: gen_robot_name(""),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = gen_robot_name(&self.name);
    }
}

fn gen_robot_name(old_name: &str) -> String {
    let mut name = String::with_capacity(ROBOT_NAME_LEN);

    for i in 0..ROBOT_NAME_LEN {
        let modulo: u8;
        let ascii_offset: u8;

        if i < 2 {
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
