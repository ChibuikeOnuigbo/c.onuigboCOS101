use std::io;

// A small helper: read user input safely
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

        println!(" Invalid input. Please try again.");
    }
}

// Safe integer parsing
fn get_number(prompt: &str) -> u32 {
    loop {
        let input = get_input(prompt);

        if let Ok(num) = input.parse::<u32>() {
            return num;
        }

        println!(" Please enter a valid number.");
    }
}

fn main() {
    println!("=== APS LEVEL VALIDATOR ===");

    // ALL ROLES AND POSITIONS (VECTOR DATA)
    let roles = vec!["office administrator", "academic", "lawyer", "teacher"];

    let aps_levels = vec![
        ("APS 1-2", vec!["intern", "-", "paralegal", "placement"]),
        ("APS 3-5", vec!["administrator", "research assistant", "junior associate", "classroom teacher"]),
        ("APS 5-8", vec!["senior administrator", "phd candidate", "associate", "snr teacher"]),
        ("EL1 8-10", vec!["office manager", "post-doc researcher", "senior associate 1-2", "leading teacher"]),
        ("EL2 10-13", vec!["director", "senior lecturer", "senior associate 3-4", "deputy principal"]),
        ("SES", vec!["ceo", "dean", "partner", "principal"]),
    ];

    // GET USER ROLE
    let role = loop {
        let r = get_input("Enter role (Office Administrator / Academic / Lawyer / Teacher):")
            .to_lowercase();

        if roles.contains(&r.as_str()) {
            break r;
        }
        println!(" Role not recognized. Please type exactly one valid role.");
    };

    // GET USER POSITION
    let position = get_input("Enter staff position title (e.g., Associate, Senior Lecturer):")
        .to_lowercase();

    // GET YEARS OF EXPERIENCE
    let experience = get_number("Enter years of experience:");

    // MATCH POSITION USING VECTOR TABLE
    let role_index = roles.iter().position(|&r| r == role).unwrap();

    let mut matched_level = "Not Found";

    for (level, titles) in &aps_levels {
        if position == titles[role_index].to_lowercase() {
            matched_level = level;
            break;
        }
    }

    println!("\n--------------------------------");
    println!(" Staff Role: {}", role);
    println!(" Position: {}", position);
    println!(" Years of Experience: {}", experience);
    println!(" APS Level: {}", matched_level);
    println!("--------------------------------");
}
