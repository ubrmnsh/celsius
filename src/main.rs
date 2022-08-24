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

    let selection: i32 = match selection.trim().parse() {
        Ok(num) => {
            if num == 1 || num == 2 {
                num
            } else {
                panic!("out of range");
            }
        }
        _ => panic!("out of range"),
    };

    let res = temperature_calculator(selection);
    println!("The calculated temperature is : {:?}", res)
}

fn temperature_calculator(selection: i32) -> u32 {
    let mut temp = String::new();
    println!("Enter the temperature");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line. \n");
    let temp: u32 = match temp.trim().parse() {
        Ok(num) => num,
        _ => panic!("not desired"),
    };
    if selection == 1 {
        temp + 32
    } else {
        temp - 32
    }
}
