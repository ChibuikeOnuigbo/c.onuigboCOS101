use std::io;
use std::fs; 
//Run wit Cargo run due to the sql files not in same dir with main.rs
fn main() {
    println!("Welcome to Globacom DB Manager");

    loop {
        println!("\n---------------------------------");
        println!("Who are you? (Enter a number):");
        println!("1. Administrator   (database_structure.sql)");
        println!("2. Project Manager (project_tb.sql)");
        println!("3. Employee        (staff_tb.sql)");
        println!("4. Customer        (customer_tb.sql)");
        println!("5. Vendor          (dataplan_tb.sql)");
        println!("(Type 'exit' to quit)");
        println!("---------------------------------");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        // Allow the user to quit easily
        if input == "exit" {
            println!("Goodbye!");
            break;
        }

        let filename = match input {
            "1" => "database_structure.sql",
            "2" => "project_tb.sql",
            "3" => "staff_tb.sql",
            "4" => "customer_tb.sql",
            "5" => "dataplan_tb.sql",
            _ => {
                println!("Invalid number. Please type 1, 2, 3, 4, or 5.");
                continue; // Skip the rest and ask again
            }
        };

        // Read and print the file in one go
        match fs::read_to_string(filename) {
            Ok(contents) => {
                println!("\n--- FILE CONTENT: {} ---", filename);
                println!("{}", contents);
            }
            Err(_) => {
                println!("Error: Could not find '{}'.", filename);
                println!("Make sure you ran the 'pg_dump' commands in the right folder!");
            }
        }
    }
}