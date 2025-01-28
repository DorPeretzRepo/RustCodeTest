use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Election {
    id: u32,
    description: String,
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Choice {
    id: u32,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Vote {
    contest_id: u32,
    choice_id: u32,
}

#[derive(Serialize, Debug)]
struct ResultData {
    contest_id: u32,
    total_votes: u32,
    results: Vec<ChoiceResult>,
    winner: Option<Choice>,
}

#[derive(Serialize, Debug)]
struct ChoiceResult {
    choice_id: u32,
    total_count: u32,
}

fn tally_votes(election: &Election, votes: Vec<Vote>) -> ResultData {
    let mut vote_counts: HashMap<u32, u32> = HashMap::new();

    // Filter votes to only include those matching the election ID
    for vote in votes.into_iter().filter(|v| v.contest_id == election.id) {
        if election.choices.iter().any(|c| c.id == vote.choice_id) {
            *vote_counts.entry(vote.choice_id).or_insert(0) += 1;
        }
    }

    let total_votes = vote_counts.values().sum();

    let mut results: Vec<ChoiceResult> = election.choices.iter().map(|choice| {
        ChoiceResult {
            choice_id: choice.id,
            total_count: *vote_counts.get(&choice.id).unwrap_or(&0),
        }
    }).collect();

    results.sort_by(|a, b| b.total_count.cmp(&a.total_count));

    let winner = results.first().and_then(|r| {
        if r.total_count > 0 {
            election.choices.iter().find(|c| c.id == r.choice_id).cloned()
        } else {
            None
        }
    });

    ResultData {
        contest_id: election.id,
        total_votes,
        results,
        winner,
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let election_data = fs::read_to_string("election.json")?;
    let votes_data = fs::read_to_string("votes.json")?;

    let election: Election = serde_json::from_str(&election_data)?;
    let votes: Vec<Vote> = votes_data.lines().map(|line| serde_json::from_str(line).unwrap()).collect();

    let result = tally_votes(&election, votes);

    let result_json = serde_json::to_string_pretty(&result)?;
    fs::write("result.json", result_json)?;

    println!("Tallying completed. Results written to result.json.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_choices() {
        let election = Election {
            id: 1,
            description: "Empty Election".to_string(),
            choices: vec![],
        };

        let votes = vec![Vote { contest_id: 1, choice_id: 1 }];
        let result = tally_votes(&election, votes);

        assert_eq!(result.total_votes, 0);
        assert!(result.results.is_empty());
        assert!(result.winner.is_none());
    }

    #[test]
    fn test_tied_votes() {
        let election = Election {
            id: 1,
            description: "Tied Election".to_string(),
            choices: vec![
                Choice { id: 1, text: "Option A".to_string() },
                Choice { id: 2, text: "Option B".to_string() },
            ],
        };

        let votes = vec![
            Vote { contest_id: 1, choice_id: 1 },
            Vote { contest_id: 1, choice_id: 2 },
        ];
        let result = tally_votes(&election, votes);

        assert_eq!(result.total_votes, 2);
        assert_eq!(result.results.len(), 2);
        assert!(result.winner.is_some());
    }

    #[test]
    fn test_invalid_votes() {
        let election = Election {
            id: 1,
            description: "Invalid Votes".to_string(),
            choices: vec![
                Choice { id: 1, text: "Valid Option".to_string() },
            ],
        };

        let votes = vec![
            Vote { contest_id: 1, choice_id: 99 },
        ];
        let result = tally_votes(&election, votes);

        assert_eq!(result.total_votes, 0);
        assert_eq!(result.results[0].total_count, 0);
        assert!(result.winner.is_none());
    }

    #[test]
    fn test_multiple_contests() {
        let election = Election {
            id: 1,
            description: "Election One".to_string(),
            choices: vec![
                Choice { id: 1, text: "Option A".to_string() },
            ],
        };

        let votes = vec![
            Vote { contest_id: 2, choice_id: 1 },
        ];
        let result = tally_votes(&election, votes);

        assert_eq!(result.total_votes, 0);
        assert!(result.results.iter().all(|r| r.total_count == 0));
        assert!(result.winner.is_none());
    }

    #[test]
    fn test_missing_fields() {
        let invalid_json = "{ \"id\": 1 }"; // Missing fields

        let parsed_result: Result<Election, _> = serde_json::from_str(invalid_json);

        assert!(parsed_result.is_err(), "Expected an error when parsing incomplete JSON.");
    }

    #[test]
    fn test_duplicate_choice_ids() {
        let election = Election {
            id: 1,
            description: "Duplicate Choices".to_string(),
            choices: vec![
                Choice { id: 1, text: "Option A".to_string() },
                Choice { id: 1, text: "Option B".to_string() },
            ],
        };

        let votes = vec![Vote { contest_id: 1, choice_id: 1 }];
        let result = tally_votes(&election, votes);

        assert_eq!(result.total_votes, 1);
        assert_eq!(result.results.len(), 2); // Both entries with ID 1 should be counted
    }

    #[test]
    fn test_empty_files() {
        let election = Election {
            id: 1,
            description: "Empty Files Test".to_string(),
            choices: vec![],
        };

        let votes: Vec<Vote> = vec![];
        let result = tally_votes(&election, votes);

        assert_eq!(result.total_votes, 0);
        assert!(result.results.is_empty());
        assert!(result.winner.is_none());
    }

    #[test]
    fn test_single_vote() {
        let election = Election {
            id: 1,
            description: "Single Vote Test".to_string(),
            choices: vec![Choice { id: 1, text: "Option A".to_string() }],
        };

        let votes = vec![Vote { contest_id: 1, choice_id: 1 }];
        let result = tally_votes(&election, votes);

        assert_eq!(result.total_votes, 1);
        assert_eq!(result.winner.unwrap().id, 1);
    }

    #[test]
    fn test_votes_for_nonexistent_contest() {
        let election = Election {
            id: 1,
            description: "Nonexistent Contest Test".to_string(),
            choices: vec![Choice { id: 1, text: "Option A".to_string() }],
        };

        let votes = vec![Vote { contest_id: 2, choice_id: 1 }];
        let result = tally_votes(&election, votes);

        assert_eq!(result.total_votes, 0);
        assert!(result.winner.is_none());
    }
}
