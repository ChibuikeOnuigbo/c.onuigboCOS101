fn main () {
    // Using Vec::new ()
    let v: Vec < i64 > = Vec::new ();

    // printing the size of vector
    println!("The length of Vec:new is: {}", v.len());

    // Using macro
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    // println!g the size of vector
    println!("The length of macro is: {}", v.len());
}
