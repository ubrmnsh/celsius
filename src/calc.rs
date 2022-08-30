use std::io;

pub fn temperature_calculator(selection: i32) -> f32 {
    let mut temp = String::new();
    println!("Enter the temperature");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line. \n");
    let temp: u32 = match temp.trim().parse() {
        Ok(num) => num,
        _ => panic!("not desired"),
    };
    // fahrenheit to celsius
    if selection == 1 {
        ((temp - 32) * 5/9) as f32
    } else {
        ((temp * 9/5 ) + 32) as f32
    }
}
