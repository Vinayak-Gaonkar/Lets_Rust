use std::io;

fn main() {
    loop {
        let mut fahren: String = String::new();
        println!("Please enter temperature in Faran haet:");
        io::stdin()
            .read_line(&mut fahren)
            .expect("Failed to read line");
        let _heat: i32 = match fahren.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let  f_heat:i32=(_heat*(3/2))+32;
        println!("{_heat} degree of heat is {f_heat} of faranheat")
    }
    // let _x=15;
    // let _x="String variable";

    // let mut y=10;
    // y=100;
    // println!("this is x: {_x} and this is y {y}")
}
