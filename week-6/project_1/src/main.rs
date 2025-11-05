use std::io;

fn main() {
    println!("==================== MENU ====================");
    println!("P = Pounded Yam & Edinkaiko Soup  - ₦3200");
    println!("F = Fried Rice & Chicken          - ₦3000");
    println!("A = Amala & Ewedu Soup            - ₦2500");
    println!("E = Eba & Egusi Soup              - ₦2000");
    println!("W = White Rice & Stew             - ₦2500");
    println!("==============================================");

    // Ask for food type
    println!("Enter the type of food (P, F, A, E, W):");
    let mut food_type = String::new();
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");

    let food_type = food_type.trim().to_uppercase();

    // Check if the food type is valid right away
    let price = match food_type.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("\n❌ Invalid food type selected. Please restart and choose from (P, F, A, E, W).");
            return;
        }
    };

    // Ask for quantity only if food type is valid
    println!("Enter quantity:");
    let mut qty_input = String::new();
    io::stdin()
        .read_line(&mut qty_input)
        .expect("Failed to read input");

    let quantity: i32 = match qty_input.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            println!("\n❌ Invalid quantity. Please enter a positive number next time.");
            return;
        }
    };

    let mut total = price * quantity;
    // Apply 5% discount if total > ₦10,000
    if total > 10_000 {
        let discount = total as f32 * 0.05;
        total = (total as f32 - discount) as i32;
        println!("Discount Applied (5%): ₦{:.0}", discount);
    }

    println!("----------------CALCULATING TOTAL PRICE------------------------------");
    println!("Final Total: ₦{}", total);
    println!("Thank you for your order!");
}
