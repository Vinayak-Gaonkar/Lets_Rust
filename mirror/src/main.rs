use std::io;

fn main() {
    let mut input_num:String = String::new();

    io::stdin()
        .read_line(&mut input_num)
        .expect("Failed to read line");

    println!("{input_num}");
}
