use std::io;
fn main() {
    println!("Annual incentive calculator");
    println!("++++++++++++++++++++++++++++++++++++");
    println!("====================================");
    //note the functions are set to return a type for this variables
    /* If the employee is experienced and his/her age is equal to or more than 40, then the incentive of the employee is N1,560,000.
    If the employee is experienced and his/her age is equal to or more than 30 but less than 40, then the incentive should be N1,480,000.
    For experienced employee below 28 years of age the incentive should be N1,300,000 per month.
    For inexperienced employee the incentive should be N100,000.
    */
    let experience = read_ans("Are you experienced:");
    let salary = read_number("Enter your Age:");
    println!("So you are {} years old", salary);
    println!("You answered for experience: {}", experience);
    if experience == "y" && salary >= 40{
        println!("Your annual incentive is N1,560,000");
    }
    else if experience == "y" && salary >=30 && salary <40{
        println!("Your annual incentive is N1,480,000");
    }
    else if experience == "y" && salary <28{
        println!("Your annual incentive is N1,300,000");
    }
    else{
        println!("Your annual incentive is N100,000");
    }
}
fn read_number(prompt: &str) -> i32 {
    //so here we will read the input from the user
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line. Pls try again.");
        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please type a valid number!!\n");
                continue;
            }
        }
    }
}
fn read_ans(prompt: &str) -> String {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line. Please try again.");

        let trimmed = input.trim(); // remove spaces and newline (\n)

        if trimmed.len() == 1 {
            let ch = trimmed.chars().next().unwrap(); // ✅ get first character

            if ch == 'y' || ch == 'y' || ch == 'n' || ch == 'N' {
                return ch.to_string().to_lowercase(); // ✅ convert char to String and return
            } else {
                println!("Please type only 'Y' or 'N'!\n");
                continue;
            }
        } else {
            println!("⚠️ Enter a single character — 'Y' or 'N' only!\n");
            continue;
        }
    }
}
