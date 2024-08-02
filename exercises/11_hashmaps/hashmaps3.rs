use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // Update the score for team 1
        let team_1 = scores.entry(team_1_name.to_string()).or_insert(Team::default());
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        // Update the score for team 2
        let team_2 = scores.entry(team_2_name.to_string()).or_insert(Team::default());
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2\nFrance,Italy,3,1\nPoland,Spain,2,0\nGermany,England,2,1\nEngland,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
