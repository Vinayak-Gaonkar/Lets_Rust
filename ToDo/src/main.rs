use std::io;

mod globals {
    pub static mut GLOBAL_TODO_LIST: Vec<String> = Vec::new();
}

fn main() {
    loop {
        let mut input_value = String::new();
        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read line");

        input_value = input_value.trim_end().to_string();
        let split_value = input_value.split(":");

        let collection: Vec<&str> = split_value.collect();
        if collection[0]=="exit"{
            break;
        }
        if collection[0] == "add" {
            unsafe{
                globals::GLOBAL_TODO_LIST.push(collection[1].to_string())
            }
        }
    };

    println!("{:?}", unsafe { &globals::GLOBAL_TODO_LIST });
}
