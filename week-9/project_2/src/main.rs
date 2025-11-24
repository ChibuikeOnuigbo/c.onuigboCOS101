fn main() {
    // 1. Define the separate datasets as requested in the prompt
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // 2. Call the merge function with error handling logic
    println!("MERGING DATASETS FOR EFCC RECORD...");
    match merge_datasets(&names, &ministries, &zones) {
        Ok(merged_data) => {
            // 3. Display the Output nicely
            println!("\n{:<5} | {:<25} | {:<20} | {:<15}", "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE");
            println!("{:-<75}", "-"); // Divider line
            
            for (index, record) in merged_data.iter().enumerate() {
                println!(
                    "{:<5} | {:<25} | {:<20} | {:<15}",
                    index + 1,
                    record.name,
                    record.ministry,
                    record.zone
                );
            }
            println!("\nData Merge Successful.");
        }
        Err(e) => {
            eprintln!("Error merging datasets: {}", e);
        }
    }
}

// A struct to hold the merged information cleanly
struct Commissioner {
    name: String,
    ministry: String,
    zone: String,
}

// Function to merge the vectors with basic error checking
fn merge_datasets(
    names: &Vec<&str>, 
    ministries: &Vec<&str>, 
    zones: &Vec<&str>
) -> Result<Vec<Commissioner>, String> {
    
    // Error Handling: Ensure all datasets are of equal length
    if names.len() != ministries.len() || names.len() != zones.len() {
        return Err("Datasets have mismatched lengths. Cannot merge safely.".to_string());
    }

    let mut merged_list: Vec<Commissioner> = Vec::new();

    for i in 0..names.len() {
        let new_record = Commissioner {
            name: names[i].to_string(),
            ministry: ministries[i].to_string(),
            zone: zones[i].to_string(),
        };
        merged_list.push(new_record);
    }

    Ok(merged_list)
}