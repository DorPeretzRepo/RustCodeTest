# RustCodeTest

# Election Tallying CLI

This is a Rust CLI tool for tallying votes in a single-winner election using the first-past-the-post method.

## Features
- Accepts election data and vote data in JSON format.
- Outputs the results in JSON format, including the winner.
- Includes comprehensive unit tests for various scenarios.

## Requirements
- Rust (latest stable version recommended)

## Setup and Usage

1. Clone the repository:
    ```bash
    git clone <repository-url>
    cd <repository-folder>
    ```

2. Add your election data to `election.json` and vote data to `votes.json`. Example formats are provided below:

   **election.json:**
   ```json
   {
       "id": 1,
       "description": "Best Programming Language",
       "choices": [
           {"id": 1, "text": "Rust"},
           {"id": 2, "text": "Python"},
           {"id": 3, "text": "Go"}
       ]
   }
   ```

   **votes.json:**
   ```json
   {"contest_id": 1, "choice_id": 1}
   {"contest_id": 1, "choice_id": 2}
   {"contest_id": 1, "choice_id": 1}
   {"contest_id": 1, "choice_id": 3}
   ```

3. Build the project:
    ```bash
    cargo build --release
    ```

4. Run the executable:
    ```bash
    ./target/release/rust_tally_functionality
    ```

5. Results will be written to `result.json`.

## Testing

This project includes comprehensive unit tests to ensure functionality and robustness. Key test cases include:

- **Basic Functionality:** Verifies correct vote tallying and winner selection.
- **Edge Cases:**
   - No choices in the election.
   - Single vote cast.
   - Votes for a nonexistent contest.
   - Tied votes among two or more choices.
- **Invalid Input:**
   - Malformed JSON (e.g., missing fields or incorrect structure).
   - Votes for nonexistent choices.
- **Duplicate Choices:** Handles elections with duplicate `choice_id`s gracefully.
- **Empty Files:** Ensures correct behavior with empty `election.json` or `votes.json`.

### Run Tests
To run the unit tests, execute:
```bash
cargo test
```

## Example Output
```json
{
    "contest_id": 1,
    "total_votes": 4,
    "results": [
        {"choice_id": 1, "total_count": 2},
        {"choice_id": 2, "total_count": 1},
        {"choice_id": 3, "total_count": 1}
    ],
    "winner": {"choice_id": 1, "text": "Rust"}
}
```

## Code Documentation
Generate the Rustdoc documentation by running:
```bash
cargo doc --open
```
