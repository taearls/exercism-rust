use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut result = String::from("Team                           | MP |  W |  D |  L |  P");
    if match_results.len() > 0 {
        let mut teams_hashmap: HashMap<&str, Team> = HashMap::new();
        let mut tournament_matches_vec: Vec<&str> = match_results.split("\n").collect();
        // don't need to analyze the default string.
        tournament_matches_vec.pop();

        for tournament_match in tournament_matches_vec {
            let match_result: Vec<&str> = tournament_match.split(";").collect();
            let first_team_name = match_result.get(0).unwrap();
            let second_team_name = match_result.get(1).unwrap();
            if let None = teams_hashmap.get(first_team_name) {
                teams_hashmap.insert(first_team_name, Team::new());
            }
            if let None = teams_hashmap.get(second_team_name) {
                teams_hashmap.insert(second_team_name, Team::new());
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
                _ => panic!("invalid input"),
            }
        }

        // TODO: now that I have all the data mapped out, I need to add a sorted version of it to the result string slice
    }
    result
}

pub struct Team {
    matches_played: usize,
    matches_won: usize,
    matches_drawn: usize,
    matches_lost: usize,
    points: usize,
}

impl Team {
    pub fn new() -> Self {
        Team {
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
}
