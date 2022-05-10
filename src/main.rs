use std::io;

fn main() {

    loop {
        println!("Please input the temperature in F that you would like converted to Celsius");

        let mut f = String::new();

        io::stdin()
            .read_line(&mut f)
            .expect("Failed to read line");

        let f: u32 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        let c = (f - 32) * 5/9;
        println!("Celsius temperature: {c}");
    }
}