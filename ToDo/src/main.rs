use std::io;


fn main() {
    let mut input_value: String = String::new();
    let mut todo_list : Vec<&str> = Vec::new();

    io::stdin()
    .read_line(&mut input_value)
    .expect("Failed to read line");

    input_value = input_value.trim_end().to_string();
    let mut split_value = input_value.split(":");

    let collection: Vec<&str> = split_value.collect();
    if collection[0]=="add" {
        todo_list.push(collection[1])
    }


    println!("{:?}", todo_list);
}
