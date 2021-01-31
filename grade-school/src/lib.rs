pub struct School {
    students: Vec<(u32, String)>,
}

impl School {
    pub fn new() -> School {
        School { students: vec![] }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.push((grade, student.to_string()));
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();

        for (grade, _student) in &self.students {
            if !result.contains(grade) {
                result.push(*grade);
            }
        }
        result.sort_unstable();
        result
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for (student_grade, student) in &self.students {
            if *student_grade == grade {
                result.push(String::from(student))
            }
        }
        result.sort_unstable();
        result
    }
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}