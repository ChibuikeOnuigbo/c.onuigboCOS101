fn main() {
    let principal:f64 = 520_000_000.0; // N520,000,000
    let rate:f64 = 10.0;               // 10% per annum
    let time:f64 = 5.0;                // 5 years

    // Calculate amount after 5 years
    let amount = principal * (1.0 + (rate / 100.0)).powf(time);
    let compound_interest = amount - principal;

    println!("--- Compound Interest Calculator ---");
    println!("Principal (P): N{:.2}", principal);
    println!("Rate (R): {:.2}%", rate);
    println!("Time (n): {:.0} years", time);
    println!("Amount after 5 years (A): N{:.2}", amount);
    println!("Compound Interest (CI): N{:.2}", compound_interest);
}
