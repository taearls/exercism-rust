use std::collections::HashMap;
use std::fmt;
use std::cmp::Ordering;

pub fn tally(match_results: &str) -> String {
    let mut result = String::from("Team                           | MP |  W |  D |  L |  P");
    if match_results.len() > 0 {
        let mut teams_hashmap: HashMap<&str, Team> = HashMap::new();
        let tournament_matches_vec: Vec<&str> = match_results.split("\n").collect();

        for tournament_match in tournament_matches_vec {
            let match_result: Vec<&str> = tournament_match.split(";").collect();
            let first_team_name = match_result.get(0).unwrap();
            let second_team_name = match_result.get(1).unwrap();
            if let None = teams_hashmap.get(first_team_name) {
                teams_hashmap.insert(first_team_name, Team::new(first_team_name));
            }
            if let None = teams_hashmap.get(second_team_name) {
                teams_hashmap.insert(second_team_name, Team::new(second_team_name));
            }

            match match_result.get(2).unwrap() {
                &"win" => {
                    Team::win(teams_hashmap.get_mut(first_team_name).unwrap());
                    Team::lose(teams_hashmap.get_mut(second_team_name).unwrap());
                },
                &"loss" => {
                    Team::lose(teams_hashmap.get_mut(first_team_name).unwrap());
                    Team::win(teams_hashmap.get_mut(second_team_name).unwrap());
                },
                &"draw" => {
                    Team::draw(teams_hashmap.get_mut(first_team_name).unwrap());
                    Team::draw(teams_hashmap.get_mut(second_team_name).unwrap());
                },
                _ => panic!("invalid input. has to be win, loss, or draw."),
            }
        }
        // sort hashmap by points, then by team name alphabetically
        let sorted_vec: Vec<Team> = Team::get_sorted_vec(teams_hashmap);

        // add each team in sorted order to the result String
        for match_result in sorted_vec.iter() {
            result.push_str(&format!("{}", match_result));
        }
    }
    result
}

pub struct Team {
    name: String,
    matches_played: usize,
    matches_won: usize,
    matches_drawn: usize,
    matches_lost: usize,
    points: usize,
}

impl Team {
    pub fn new(name: &str) -> Self {
        Team {
            name: String::from(name),
            matches_played: 0,
            matches_won: 0,
            matches_lost: 0,
            matches_drawn: 0,
            points: 0,
        }
    }
    pub fn win(&mut self) {
        self.matches_played += 1;
        self.matches_won += 1;
        self.points += 3;
    }
    pub fn lose(&mut self) {
        self.matches_played += 1;
        self.matches_lost += 1;
    }
    pub fn draw(&mut self) {
        self.matches_played += 1;
        self.matches_drawn += 1;
        self.points += 1;
    }
    pub fn get_sorted_vec(hashmap: HashMap<&str, Team>) -> Vec<Team> {
        let mut result: Vec<Team> = Vec::with_capacity(hashmap.len());

        // build up an unsorted vec from the hashmap
        for (key, val) in hashmap.iter() {
            let team = Team {
                name: key.to_string(),
                matches_played: val.matches_played,
                matches_won: val.matches_won,
                matches_drawn: val.matches_drawn,
                matches_lost: val.matches_lost,
                points: val.points,
            };
            result.push(team);
        }

        // sort by points
        // TODO: sort by name alphabetically if points are the same
        result.sort_by(|a, b| {
            let mut cmp = b.points.cmp(&a.points);
            if cmp == Ordering::Equal {
                cmp = a.name.cmp(&b.name);
            }
            cmp
        });
        result
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // https://doc.rust-lang.org/std/fmt/index.html#fillalignment
        write!(f, "\n{:<30} |{:>3} |{:>3} |{:>3} |{:>3} |{:>3}", self.name, self.matches_played, self.matches_won, self.matches_drawn, self.matches_lost, self.points)
    }
}