use std::fs::File;
use std::io::{Write, Result};

// Define the structure for a Student
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() -> Result<()> {
    // 1. Initialize the data in a Vector
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328828"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: 200,
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    // 2. Display details to the console
    println!("PAU SMIS - Student Data Display");
    println!("==========================================");
    for student in &students {
        println!(
            "Name: {}, Matric: {}, Dept: {}, Level: {}",
            student.name, student.matric_number, student.department, student.level
        );
    }

    // 3. Save details to a file (CSV format for Excel compatibility)
    save_to_csv(&students)?;

    println!("\nData successfully saved to 'pau_smis.csv'.");
    
    Ok(())
}

// Function to handle file writing with error propagation
fn save_to_csv(students: &Vec<Student>) -> Result<()> {
    let mut file = File::create("pau_smis.csv")?;

    // Write the Header
    writeln!(file, "Student Name,Matric. Number,Department,Level")?;

    // Write the data rows
    for student in students {
        writeln!(
            file,
            "{},{},{},{}",
            student.name, student.matric_number, student.department, student.level
        )?;
    }

    Ok(())
}