use std::io;

fn main() {
    let mut n:String = String::new();
    println!("Please input your guess");
    
    io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
    
    let n: u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    fibonacci(n)
}

fn fibonacci(n:u64){
    let mut cur:u64=0;
    let mut prev:u64 = 1;
    let mut prev1:u64 = 0;

    while cur<n {
        println!("{cur}");
        cur=prev+prev1;
        prev1=prev;
        prev=cur;
        
    }
}
