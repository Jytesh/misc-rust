use std::error::Error;

mod digest;
mod password_hasher;
fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = std::env::args().collect();

    match args.get(1) {
        Some(x) => {
            match &x[..] {
                "1" => {
                    digest::main(args)?;
                }
                "2" => {
                    password_hasher::main(args);
                }
                &_ => {
                    panic!("Unknown type!");
                }
            }
        },
        _ => {
            eprintln!("Couldn't parse arguments!!!");
            panic!()
        }
    };
    Ok(())
}

