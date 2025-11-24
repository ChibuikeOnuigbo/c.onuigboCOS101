use std::fs::File;
use std::io::{Write, Result};

fn main() {
    // 1. Define the drink categories as Vectors
    let lager = vec![
        "33 Export", 
        "Desperados", 
        "Goldberg", 
        "Gulder", 
        "Heineken", 
        "Star"
    ];

    let stout = vec![
        "Legend", 
        "Turbo King", 
        "Williams"
    ];

    let non_alcoholic = vec![
        "Maltina", 
        "Amstel Malta", 
        "Malta Gold", 
        "Fayrouz"
    ];

    // 2. Call the file generation function with error handling
    println!("Generating Nigerian Breweries Product File...");
    
    match save_drinks_to_file(&lager, &stout, &non_alcoholic) {
        Ok(_) => println!("Success! File 'nigerian_breweries_products.csv' created."),
        Err(e) => eprintln!("Error creating file: {}", e),
    }
}

// Function to write the data to a file
fn save_drinks_to_file(
    lager: &Vec<&str>, 
    stout: &Vec<&str>, 
    non_alcoholic: &Vec<&str>
) -> Result<()> {
    
    let mut file = File::create("nigerian_breweries_products.csv")?;

    // Write the Header Row
    writeln!(file, "Lager,Stout,Non-Alcoholic")?;

    // Determine the maximum length among the vectors to avoid index out of bounds
    // Lager has 6 items, Stout has 3, Non-Alcoholic has 4. Max is 6.
    let max_len = lager.len().max(stout.len()).max(non_alcoholic.len());

    // Iterate up to the maximum length
    for i in 0..max_len {
        // Safe access: If 'i' is within bounds, get the drink. If not, use empty string.
        let l_item = if i < lager.len() { lager[i] } else { "" };
        let s_item = if i < stout.len() { stout[i] } else { "" };
        let n_item = if i < non_alcoholic.len() { non_alcoholic[i] } else { "" };

        // Write the row to the CSV file
        writeln!(file, "{},{},{}", l_item, s_item, n_item)?;
    }

    Ok(())
}