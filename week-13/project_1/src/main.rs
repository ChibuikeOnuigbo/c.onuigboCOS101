use std::io;
use std::io::Read;
use std::io::Write; 
use std::fs::File;

fn main() {
    generate_data_files();

    println!("--------------------------");
    println!("   GLOBACOM DB MANAGER    ");
    println!("--------------------------");
    println!("Who are you? (Type the number):");
    println!("1. Administrator");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");
    println!("--------------------------");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let role = input.trim();

    match role {
        "1" => read_file("database_structure.sql"),
        "2" => read_file("project_tb.sql"),
        "3" => read_file("staff_tb.sql"),
        "4" => read_file("customer_tb.sql"),
        "5" => read_file("dataplan_tb.sql"),
        _ => println!("Invalid number. Please type 1, 2, 3, 4, or 5."),
    }
}

fn generate_data_files() {
    let admin_data = "
-----------------------------------------------------------------------
|                      DATABASE STRUCTURE SUMMARY                     |
-----------------------------------------------------------------------
| Table Name    | Primary Key | Columns                               |
|---------------|-------------|---------------------------------------|
| Staff         | staff_id    | staff_name, dno, staff_sal, age...    |
| Department    | dno         | dname, dlocation, pno, manager_id...  |
| Project       | pno         | pname, pduration, project_managerid   |
| Customer      | c_id        | c_name, c_age, c_email, c_mobile...   |
| Dataplan      | data_id     | data_size, data_duration, data_price  |
-----------------------------------------------------------------------
";
    write_to_file("database_structure.sql", admin_data);

    let project_data = "
--------------------------------------------------------------
| Pno | Pname | Pduration | Project_Managerid                |
|-----|-------|-----------|----------------------------------|
| 11  | A     | 9 Months  | 102                              |
| 22  | B     | 14 Months | 97                               |
| 33  | C     | 16 Months | 120                              |
| 44  | D     | 25 Months | 108                              |
| 55  | E     | 9 Months  | 107                              |
--------------------------------------------------------------
";
    write_to_file("project_tb.sql", project_data);

    let staff_data = "
--------------------------------------------------------------------------------
| staff_id | staff_name     | dno | staff_sal  | age | mobile      |
|----------|----------------|-----|------------|-----|-------------|
| 101      | Alade Joy      | 2   | 250000     | 33  | 8023089832  |
| 100      | Mustapha Ali   | 3   | 175000     | 32  | 8063285831  |
| 107      | Alokwe Martin  | 7   | 380000     | 48  | 7090082812  |
| 97       | Dankade Aminat | 5   | 550000     | 40  | 9023688832  |
| 108      | Josiah Joshua  | 1   | 120000     | 30  | 8053189131  |
| 102      | Mankinde Mary  | 2   | 450000     | 55  | 9023487830  |
| 120      | Adeleke Jane   | 4   | 200000     | 38  | 7061045862  |
| 122      | Osahon Mark    | 6   | 320000     | 44  | 8022289842  |
| 117      | Suleman Ajayi  | 3   | 800000     | 50  | 70300899811 |
| 104      | Kuti Lawal     | 1   | 750000     | 35  | 9145689842  |
--------------------------------------------------------------------------------
";
    write_to_file("staff_tb.sql", staff_data);

    let customer_data = "
----------------------------------------------------------------------------------------------------------
| C_id | C_name          | C_age | C_email              | C_mobile    | Eid | Data_id |
|------|-----------------|-------|----------------------|-------------|-----|---------|
| 110  | Musta Karim     | 35    | m_karim@gmail.com    | 08055089112 | 102 | 5       |
| 111  | Lilian Jaiya    | 43    | l_jaiye@gmail.com    | 08055185341 | 100 | 3       |
| 112  | Arthur Musa     | 50    | a_musa@gmail.com     | 07055282813 | 107 | 10      |
| 113  | Philip Akonjo   | 41    | p_akonjo@gmail.com   | 09052356772 | 100 | 2       |
| 114  | Marylene Mapa   | 33    | m_mapa@gmail.com     | 08053333551 | 120 | 5       |
| 115  | Oghenero Agor   | 50    | o_agor@gmail.com     | 07055566774 | 117 | 11      |
| 116  | Adams Bree      | 33    | a_bree@gmail.com     | 08056765424 | 102 | 1       |
| 117  | Okafor Mathias  | 45    | o_mathias@gmail.com  | 08056763367 | 120 | 10      |
| 118  | Samson Adeleke  | 65    | s_adeleke@gmail.com  | 07056774423 | 117 | 11      |
| 119  | Lawal Tamire    | 35    | l_tamire@gmail.com   | 09052111101 | 107 | 5       |
| 120  | James Job       | 44    | j_job@gmail.com      | 08059693919 | 100 | 8       |
| 121  | Matthew Jakande | 21    | m_jakande@gmail.com  | 07051232144 | 120 | 2       |
| 122  | Jimila Adegboye | 20    | j_adegboye@gmail.com | 08054921923 | 107 | 5       |
----------------------------------------------------------------------------------------------------------
";
    write_to_file("customer_tb.sql", customer_data);

    let vendor_data = "
--------------------------------------------------------------
| Data_id | Data_size | Data_duration (days) | Data_price (naira) |
|---------|-----------|----------------------|--------------------|
| 1       | 350MB     | 2                    | 200                |
| 2       | 1.8GB     | 14                   | 500                |
| 3       | 3.9GB     | 30                   | 1000               |
| 4       | 7.5GB     | 30                   | 1500               |
| 5       | 9.2GB     | 30                   | 2000               |
| 6       | 10.8GB    | 30                   | 2500               |
| 7       | 14GB      | 30                   | 3000               |
| 8       | 18GB      | 30                   | 4000               |
| 9       | 24GB      | 30                   | 5000               |
| 10      | 29.9GB    | 30                   | 8000               |
| 11      | 50GB      | 30                   | 10000              |
--------------------------------------------------------------
";
    write_to_file("dataplan_tb.sql", vendor_data);
}

fn write_to_file(filename: &str, content: &str) {
    let mut file = File::create(filename).expect("Could not create file");
    file.write_all(content.as_bytes()).expect("Could not write to file");
}

fn read_file(path: &str) {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            println!("Error: Could not find {}", path);
            return;
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    println!("{}", contents);
}
