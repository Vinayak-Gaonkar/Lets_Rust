use std::io;

struct MyStruct {
    c: u32,
    o: u32,
    d: u32,
    e: u32,
    h: u32,
    f: u32,
}

// fn generateHash(input_string:String, data:MyStruct){
//     for c in input_string.chars(){
//         if c == 'c' || c=='o' || c=='d' || c=='e' || c== 'h' || c=='f' {
//             data.
//         }
//     }
// }

fn main() {
    let mut input_num = String::new();

    io::stdin()
        .read_line(&mut input_num)
        .expect("Failed to read line");

    let input_num = input_num.trim().parse().unwrap();

    for _ in 0..input_num {
        let mut sample = String::new();
        io::stdin().read_line(&mut sample).expect("invalid input");
        let sample: i32 = sample.trim().parse().unwrap();

       let letters_arr:[char; 6]=['c','o','d','e','h','f'];
       let mut count_arr:[u32; 6]=[0,0,0,0,0,0];


        for _ in 0..sample {
            let mut input_string = String::new();
            io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read line");
            for letter in input_string.chars(){
                let index = letters_arr.iter().position(|&x| x == letter);
                
                match index {
                    Some(i)=> {
                        count_arr[i] += 1;
                    },
                    None=> println!("element not found")
                }
            }
            println!("{:?}", count_arr);
        }
    }
}
