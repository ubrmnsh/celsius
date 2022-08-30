use std::io;
mod calc;

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
                panic!("{selection} is not a valid option, Please choose between 1 or 2");
            }
        }
        _ => panic!("{selection} is not a valid option, Please choose between 1 or 2"),
    };

    let res = calc::temperature_calculator(selection);
    if selection == 1 {
        println!("temp is : {:?}° celsius", res)
    } else {
        println!("temp is : {:?}° fahrenheit", res)
    }
   // println!("The calculated temperature is : {:?}", res)
}


