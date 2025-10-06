fn main() {
    //i have some errors in this code but i was annoyed i search and i found out
    //that if i dont be specifc with  data type  rust will take some operations as ambiguous with mutiple datatype and read errors
    //I guess i hadte static programming languages
    let principal:f64 = 510_000.0; // Initial price of TV
    let rate:f64 = 5.0;            // Depreciation rate (5% per year)
    let time:f64 = 3.0;            // 3 years
    // Calculate value after 3 years
    //so i cant use ** for raising to power in rust
    //I have to use powf() method and i dont know why
    //i searched online and  found out ** is for dereference i dont want to even bother using it.
    let value = principal * (1.0 - (rate / 100.0)).powf(time);
    println!("--- TV Depreciation Calculator ---");
    println!("Original Value (P): N{:.2}", principal);
    println!("Depreciation Rate (R): {:.2}%", rate);
    println!("Time (n): {:.0} years", time);
    println!("Value After 3 Years (A): N{:.2}", value);
}
