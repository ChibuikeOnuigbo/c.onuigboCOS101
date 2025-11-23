use std::io;

// Safe string input
fn get_input(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }

        println!("Invalid input. Please try again.");
    }
}

// Safe number input
fn get_number(prompt: &str) -> u32 {
    loop {
        let input = get_input(prompt);

        if let Ok(number) = input.parse::<u32>() {
            return number;
        }

        println!("Please enter a valid number.");
    }
}

fn main() {
    println!("=== EY NIGERIA – EXPERIENCE CHECKER ===");

    let mut candidates: Vec<(String, u32)> = Vec::new();

    loop {
        let name = get_input("Enter candidate name:");
        let years = get_number("Enter years of programming experience:");

        candidates.push((name.clone(), years));

        let again = get_input("Add another candidate? (yes/no):").to_lowercase();

        if again != "yes" {
            break;
        }
    }

    // FIND MAX EXPERIENCE
    if let Some((best_name, best_years)) =
        candidates.iter().max_by_key(|(_, years)| years)
    {
        println!("--------------------------------");
        println!("Candidate with highest experience:");
        println!(" Name: {}", best_name);
        println!(" Years: {}", best_years);
        println!("--------------------------------");
    } else {
        println!(" No candidates were entered.");
    }
}
