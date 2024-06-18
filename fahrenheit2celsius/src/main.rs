use std::io;

fn main() {
    println!("Fahrenheit to Celsius.");
    loop {
        println!("Please input your fahrenheit value:");
        let mut fah_in = String::new();
        // user input
        io::stdin()
            .read_line(&mut fah_in)
            .expect("Failed to read line.");

        if fah_in.trim() == "quit" {break};
        // string to f32
        let fah_in: f32 = fah_in.trim().parse().expect("Failed to parse as f32");

        // fah to cul
        let cel = (fah_in - 32.0) * 5.0 /9.0;
        println!("{fah_in} in celsius is {cel}°");
    }
    
}
