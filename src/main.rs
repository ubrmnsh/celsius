use std::io;

fn main() {
    println!(
        "== Calculate Temperature ==>
              Choose conversation
             1. Fahrenheit to Celsius
             2. Celsius to Fahrenheit"
    );

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line. \n");

    let selection: u32 = match selection.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Out of range selection"),
    };
    //calculate_temperature(34);

    println!("chosen number is: {selection}");
}

fn calculate_temperature(temp: i32) {
    let temp = temp + 32;
    println!("calculated temperature is: {temp}");
}
