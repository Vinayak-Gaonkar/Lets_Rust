use std::io;

fn main() {
    let mut input_num: String = String::new();

    io::stdin()
        .read_line(&mut input_num)
        .expect("Failed to read line");

    let sample_input: i32 = input_num.trim().parse().unwrap();

    for _ in 0..sample_input {
        let mut turns = String::new();
        io::stdin().read_line(&mut turns).expect("Invalid turns");

        let sample: Vec<&str> = turns.split_whitespace().collect();

        if (sample[0].parse::<i32>().unwrap() + sample[0].parse::<i32>().unwrap()) < 7 {
            println!("YES");
        }else{
            println!("NO");
        }
    }
}
