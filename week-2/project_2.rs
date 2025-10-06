fn main() {
    //Pls sir i found a better method to do this project rather than creating varibles for each amt
    //It furns out rust has some similarities with python and javascript
    //So i created a tuple array to hold the data
    //honestly never though a simple work like this will be longer for rust
    let sales_data = [
        (2, 450_000.0),    // Toshiba
        (1, 1_500_000.0),  // Mac
        (3, 750_000.0),    // HP
        (3, 2_850_000.0),  // Dell
        (1, 250_000.0),    // Acer
    ];

    // Calculate the total sum of all sales amounts.
    // .map(|&(_, amount)| amount) extracts the second element (the amount) from each tuple.
    //so it turns out in rust u have to use iter() otherwise i cant loop through array i personally hate it.
    let total_amount: f64 = sales_data.iter().map(|&(_, amount)| amount).sum();

    // Calculate the total quantity of items sold.
    // .map(|&(qty, _)| qty as f64) extracts the first element (the quantity) and converts it to f64.
    let total_quantity: f64 = sales_data.iter().map(|&(qty, _)| qty as f64).sum();

    // Calculate the average price per individual item.
    let average_price_per_item = total_amount / total_quantity;

    println!("--- Sales Summary ---");
    for (i, (qty, amount)) in sales_data.iter().enumerate() {
        println!("Record {}: Qty {}, Amount N{:.2}", i + 1, qty, amount);
    }
    println!("---------------------------");
    println!("Total Sales Amount: N{:.2}", total_amount);
    println!("Total Items Sold: {:.0}", total_quantity);
    println!("Average Price Per Item: N{:.2}", average_price_per_item);
}