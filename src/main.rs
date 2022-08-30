use std::io;
mod calc;

fn main() {
    let message = " == Calculate Temperature ==
    Choose Conversation Scale 1. Fahrenheit to Celsius
                              2. Celsius to Fahrenheit";

    println!("\x1b[93m{}\x1b[0m", message);

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line. \n");

    let selection: i32 = match selection.trim().parse() {
        Ok(num) => {
            if num == 1 || num == 2 {
                num
            } else {
                panic!("\t{selection} is not a valid option, Please choose between 1 or 2");
            }
        }
        _ => panic!("\t{selection} is not a valid option, Please choose between 1 or 2"),
    };

    calc::temperature_calculator(selection);
}
