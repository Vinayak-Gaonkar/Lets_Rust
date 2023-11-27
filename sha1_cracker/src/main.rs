use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead,BufReader}
};

const SHA_HEX_STRING_LENGTH:usize=40;

fn main()-> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack=args[2].trim();

    if hash_to_crack.len() != SHA_HEX_STRING_LENGTH {
        return Err("sha hash is not valid".into());
    }

    let wordList_file=File::open(&args[1])?;
    let reader=BufReader::new(&wordList_file);

    for line in reader.lines() {
        println!("{:?}", line);
    }

    return Ok(());
}
