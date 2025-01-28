use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

/// Represents an election with its ID, description, and available choices.
#[derive(Serialize, Deserialize, Debug)]
struct Election {
    id: u32,
    description: String,
    choices: Vec<Choice>,
}

/// Represents a single choice in an election.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Choice {
    id: u32,
    text: String,
}

/// Represents a vote with a contest ID and a choice ID.
#[derive(Serialize, Deserialize, Debug)]
struct Vote {
    contest_id: u32,
    choice_id: u32,
}

/// Represents the results of an election tally.
#[derive(Serialize, Debug)]
struct ResultData {
    contest_id: u32,
    total_votes: u32,
    results: Vec<ChoiceResult>,
    winner: Option<Choice>,
}

/// Represents the tally of votes for a specific choice.
#[derive(Serialize, Debug)]
struct ChoiceResult {
    choice_id: u32,
    total_count: u32,
}

/// Tally the votes for a given election, returning the results.
///
/// - `election`: The election to tally votes for.
/// - `votes`: The list of votes to be tallied.
///
/// Returns a `ResultData` containing the results and the winner.
fn tally_votes(election: &Election, votes: &[Vote]) -> ResultData {
    let mut vote_counts: HashMap<u32, u32> = HashMap::new();

    // Filter votes to only include those matching the election ID
    for vote in votes.iter().filter(|v| v.contest_id == election.id) {
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

    let winner = if results.len() > 1 && results[0].total_count == results[1].total_count {
        None // Tie case: No winner
    } else {
        results.first().and_then(|r| {
            if r.total_count > 0 {
                election.choices.iter().find(|c| c.id == r.choice_id).cloned()
            } else {
                None
            }
        })
    };

    ResultData {
        contest_id: election.id,
        total_votes,
        results,
        winner,
    }
}

/// Main function to read input files, tally votes, and write the results to an output file.
fn main() -> Result<(), Box<dyn Error>> {
    let election_data = fs::read_to_string("election.json")?;
    let votes_data = fs::read_to_string("votes.json")?;

    let election: Election = serde_json::from_str(&election_data)?;
    let votes: Vec<Vote> = votes_data.lines().map(|line| serde_json::from_str(line).unwrap()).collect();

    let result = tally_votes(&election, &votes);

    let result_json = serde_json::to_string_pretty(&result)?;
    fs::write("result.json", result_json)?;

    println!("Tallying completed. Results written to result.json.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test 01: No Choices
    #[test]
    fn test_01_no_choices() {
        let election = Election {
            id: 1,
            description: "Empty Election".to_string(),
            choices: vec![],
        };

        let votes = vec![Vote { contest_id: 1, choice_id: 1 }];
        let result = tally_votes(&election, &votes);

        println!(
            "\nTest: No Choices\nInput Election: {}\nInput Votes: {}\nExpected Total Votes: 0\nActual: {}\nResult: {}\n",
            serde_json::to_string_pretty(&election).unwrap(),
            serde_json::to_string_pretty(&votes).unwrap(),
            serde_json::to_string_pretty(&result).unwrap(),
            if result.total_votes == 0 && result.results.is_empty() { "PASSED" } else { "FAILED" }
        );

        assert_eq!(result.total_votes, 0);
        assert!(result.results.is_empty());
        assert!(result.winner.is_none());
    }

    /// Test 02: Tied Votes
    #[test]
    fn test_02_tied_votes() {
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
        let result = tally_votes(&election, &votes);

        println!(
            "\nTest: Tied Votes\nInput Election: {}\nInput Votes: {}\nExpected Total Votes: 2\nActual: {}\nResult: {}\n",
            serde_json::to_string_pretty(&election).unwrap(),
            serde_json::to_string_pretty(&votes).unwrap(),
            serde_json::to_string_pretty(&result).unwrap(),
            if result.total_votes == 2 && result.winner.is_none() { "PASSED" } else { "FAILED" }
        );

        assert_eq!(result.total_votes, 2);
        assert_eq!(result.results.len(), 2);
        assert!(result.winner.is_none());
    }

    /// Test 03: Invalid Votes
    #[test]
    fn test_03_invalid_votes() {
        let election = Election {
            id: 1,
            description: "Invalid Votes".to_string(),
            choices: vec![
                Choice { id: 1, text: "Valid Option".to_string() },
            ],
        };

        let votes = vec![Vote { contest_id: 1, choice_id: 99 }];
        let result = tally_votes(&election, &votes);

        println!(
            "\nTest: Invalid Votes\nInput Election: {}\nInput Votes: {}\nExpected Total Votes: 0\nActual: {}\nResult: {}\n",
            serde_json::to_string_pretty(&election).unwrap(),
            serde_json::to_string_pretty(&votes).unwrap(),
            serde_json::to_string_pretty(&result).unwrap(),
            if result.total_votes == 0 && result.results[0].total_count == 0 { "PASSED" } else { "FAILED" }
        );

        assert_eq!(result.total_votes, 0);
        assert_eq!(result.results[0].total_count, 0);
        assert!(result.winner.is_none());
    }

    /// Test 04: Multiple Contests
    #[test]
    fn test_04_multiple_contests() {
        let election = Election {
            id: 1,
            description: "Election One".to_string(),
            choices: vec![
                Choice { id: 1, text: "Option A".to_string() },
            ],
        };

        let votes = vec![Vote { contest_id: 2, choice_id: 1 }];
        let result = tally_votes(&election, &votes);

        println!(
            "\nTest: Multiple Contests\nInput Election: {}\nInput Votes: {}\nExpected Total Votes: 0\nActual: {}\nResult: {}\n",
            serde_json::to_string_pretty(&election).unwrap(),
            serde_json::to_string_pretty(&votes).unwrap(),
            serde_json::to_string_pretty(&result).unwrap(),
            if result.total_votes == 0 && result.results.iter().all(|r| r.total_count == 0) { "PASSED" } else { "FAILED" }
        );

        assert_eq!(result.total_votes, 0);
        assert!(result.results.iter().all(|r| r.total_count == 0));
        assert!(result.winner.is_none());
    }

    /// Test 05: Missing Fields
    #[test]
    fn test_05_missing_fields() {
        let invalid_json = "{ \"id\": 1 }"; // Missing fields

        let parsed_result: Result<Election, _> = serde_json::from_str(invalid_json);

        println!(
            "\nTest: Missing Fields\nInput JSON: {}\nExpected: Error\nResult: {}\n",
            invalid_json,
            if parsed_result.is_err() { "PASSED" } else { "FAILED" }
        );

        assert!(parsed_result.is_err(), "Expected an error when parsing incomplete JSON.");
    }
}
