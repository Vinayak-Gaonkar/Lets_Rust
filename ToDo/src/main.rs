mod args;

use args::{TODO, Commands};
use clap::Parser;


fn main() {
    // loop {
        // let mut input_value = String::new();
        // io::stdin()
        //     .read_line(&mut input_value)
        //     .expect("Failed to read line");

        // input_value = input_value.trim_end().to_string();
        // let split_value = input_value.split(":");

        // let collection: Vec<&str> = split_value.collect();
        // if collection[0]=="exit"{
        //     break;
        // }
        // if collection[0] == "update" {
        //     unsafe{
        //         let update_index: usize = collection[1].parse().unwrap();
        //         if update_index >= globals::GLOBAL_TODO_LIST.len() {
        //             println!("Invalid index");
        //         }

        //         globals::GLOBAL_TODO_LIST[update_index]= collection[2].to_string();
        //     }
        // }
        // if collection[0] == "add" {
        //     unsafe{
        //         globals::GLOBAL_TODO_LIST.push(collection[1].to_string());
        //     }
        // }
        // if collection[0] == "remove" {
        //     unsafe{
        //         let remove_index: usize = collection[1].parse().unwrap();
        //         if remove_index >= globals::GLOBAL_TODO_LIST.len() {
        //             println!("Invalid index");
        //         }
        //         globals::GLOBAL_TODO_LIST.remove(remove_index);
        //     }
        // }
        // if collection[0] == "print" {
        //     println!("Your Top priorities are {:?}", unsafe { &globals::GLOBAL_TODO_LIST });
        // }
    // };

    // println!("Your Top priorities are {:?}", unsafe { &globals::GLOBAL_TODO_LIST });


    let args = TODO::parse();
    
    match &args.action {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {name:?}")
        }
    }
}
