//use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::error::Error;

pub fn main(args: Vec<String>) -> Result<(), Box<dyn Error>>{
    let path = match args.get(2) {
        Some(x) => x,
        _ => {
            panic!("No argument found!");
        }
    };
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let digest = digest_file(reader)?;
    println!("{:?}", digest);

    Ok(())
}


fn digest_file<R: Read>(mut reader: R) -> Result<[u8; 1024], Box<dyn Error>> {
    let mut buffer = [0; 1024];
    loop {
        if reader.read(&mut buffer)? == 0 {
            break;
        };
    }
    Ok(buffer)
}