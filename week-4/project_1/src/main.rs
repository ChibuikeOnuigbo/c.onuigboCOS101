use std::io;

fn main() {
    loop {
        /*Input the values from the keyboard
        Find the discriminant. The discriminant determines the number and nature of the roots.
        If the discriminant is positive, then there are two distinct roots.
        If the discriminant is zero, then there is exactly one real root.
        If the discriminant is negative, then there are no real roots.
         */
        //note the functions are set to return a type for this variables
        let a = read_number("Enter a number for a:");
        let b = read_number("Enter a number for b:");
        let c = read_number("Enter a number for c:");

        println!("values: a = {}, b = {}, c = {}", a, b, c);
        println!("Finding the roots of the equation:");
        let discriminant: f32 = (b * b) - 4.0 * a * c;
        println!("first we have to make sure its has real roots...");
        println!("Calculating the discriminant...");
        println!("Discriminant value is: {}", discriminant);
        if discriminant < 0.0 {
            println!("Sorry, the equation has no real roots.");
            println!("_____________________________________");
            println!("Please try again with different values.");
            continue;
        }
        //then to check if it has only one root
        if discriminant == 0.0 {
            let root = -b / (2.0 * a);
            println!("The equation has one real root: {}", root);
            println!("_____________________________________");
            println!("Eqn= {}x^2 + {}x + {}", a, b, c);
            break;
        } else {
            println!("The equation has two real roots.");
            let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
            println!("The equation has two real roots: {} and {}", root1, root2);
            println!("_____________________________________");
            println!("Eqn= {}x^2 + {}x + {}", a, b, c);
            break;
        }
    }
}

fn read_number(prompt: &str) -> f32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f32>() {
            Ok(num) => return num, // success → return the number
            Err(_) => {
                println!("Please type a valid number!!\n");
                continue; // restart loop and ask again
            }
        }
    }
}
