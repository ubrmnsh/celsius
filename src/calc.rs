use std::io;

pub fn temperature_calculator(selection: i32) -> f32 {
    let mut temp = String::new();
    println!("Enter the temperature");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line. \n");
    let temp: i32 = match temp.trim().parse() {
        Ok(num) => num,
        _ => panic!("not desired"),
    };
    if selection == 1 {
        // fahrenheit to celsius
        ((temp - 32) * 5 / 9) as f32
    } else {
        // celsiusn to fahrenheit
        ((temp * 9 / 5) + 32) as f32
    }
}
