use std::io;

// ----------------------- INPUT HELPERS -----------------------

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!(" Error reading input. Try again.");
            continue;
        }

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("back") {
            return f64::NAN; // special signal for backtracking
        }

        match trimmed.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!(" Invalid number. Type a valid number or type 'back' to return."),
        }
    }
}

// ----------------------- PAUSE FUNCTION -----------------------

fn wait_for_enter() {
    println!("Press ENTER to return to the menu...");
    let mut temp = String::new();
    let _ = io::stdin().read_line(&mut temp);
}

// ----------------------- CALCULATION FUNCTIONS -----------------------

fn area_trapezium() {
    loop {
        let h = read_number("Enter height (or type 'back' to return): ");
        if h.is_nan() { return; }

        let b1 = read_number("Enter base1 (or type 'back' to return): ");
        if b1.is_nan() { return; }

        let b2 = read_number("Enter base2 (or type 'back' to return): ");
        if b2.is_nan() { return; }

        let area = (h / 2.0) * (b1 + b2);
        println!("\n=== RESULT ===");
        println!("Area of Trapezium = {}\n", area);
        wait_for_enter();
        return;
    }
}

fn area_rhombus() {
    loop {
        let d1 = read_number("Enter diagonal1 (or type 'back' to return): ");
        if d1.is_nan() { return; }

        let d2 = read_number("Enter diagonal2 (or type 'back' to return): ");
        if d2.is_nan() { return; }

        let area = 0.5 * d1 * d2;
        println!("\n=== RESULT ===");
        println!("Area of Rhombus = {}\n", area);
        wait_for_enter();
        return;
    }
}

fn area_parallelogram() {
    loop {
        let base = read_number("Enter base (or type 'back' to return): ");
        if base.is_nan() { return; }

        let altitude = read_number("Enter altitude (or type 'back' to return): ");
        if altitude.is_nan() { return; }

        let area = base * altitude;
        println!("\n=== RESULT ===");
        println!("Area of Parallelogram = {}\n", area);
        wait_for_enter();
        return;
    }
}

fn area_cube() {
    loop {
        let side = read_number("Enter length of the side (or type 'back' to return): ");
        if side.is_nan() { return; }

        let area = 6.0 * side * side;
        println!("\n=== RESULT ===");
        println!("Area of Cube = {}\n", area);
        wait_for_enter();
        return;
    }
}

fn volume_cylinder() {
    loop {
        let r = read_number("Enter radius (or type 'back' to return): ");
        if r.is_nan() { return; }

        let h = read_number("Enter height (or type 'back' to return): ");
        if h.is_nan() { return; }

        let volume = std::f64::consts::PI * r * r * h;
        println!("\n=== RESULT ===");
        println!("Volume of Cylinder = {}\n", volume);
        wait_for_enter();
        return;
    }
}

// ----------------------- MAIN MENU -----------------------

fn main_menu() {
    let options: [&str; 6] = [
        "Area of Trapezium",
        "Area of Rhombus",
        "Area of Parallelogram",
        "Area of Cube",
        "Volume of Cylinder",
        "Exit"
    ];

    loop {
        println!("\n===== MTH 101 CALCULATOR =====\n");
        for (i, opt) in options.iter().enumerate() {
            println!("{}. {}", i + 1, opt);
        }
        println!("\nEnter your choice (1-6):");

        let mut choice = String::new();
        if let Err(_) = io::stdin().read_line(&mut choice) {
            println!(" Could not read input. Try again.");
            continue;
        }

        let trimmed = choice.trim();

        match trimmed.parse::<usize>() {
            Ok(1) => area_trapezium(),
            Ok(2) => area_rhombus(),
            Ok(3) => area_parallelogram(),
            Ok(4) => area_cube(),
            Ok(5) => volume_cylinder(),
            Ok(6) => {
                println!("Exiting... Goodbye!");
                return;
            }
            _ => println!(" Invalid choice. Please enter a number between 1 and 6."),
        }
    }
}

// ----------------------- MAIN -----------------------

fn main() {
    // let Menu_ms = String::new();
    main_menu();
}
