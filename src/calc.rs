use std::io;

pub fn temperature_calculator(selection: i32) {
    let mut temp = String::new();
    println!("\tEnter the temperature");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line. \n");
    let temp: i32 = match temp.trim().parse() {
        Ok(num) => num,
        _ => panic!("not desired"),
    };

    if selection == 1 {
        // fahrenheit to celsius
        let temp = ((temp - 32) * 5 / 9) as f32;
        println!("\t\tTemperature is : {:?}°C", temp)
    } else {
        // celsiusn to fahrenheit
        let temp = ((temp * 9 / 5) + 32) as f32;
        println!("\t\tTemperatures : {:?}°F", temp)
    }
}
