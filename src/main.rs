use std::io;

mod auto_sample_pass;
mod self_sampled;
fn main() {
    loop {
        let mut input = String::new();
        println!("What lib do you want to use?\n1.Pre-Sampled\n2.Self Sampled\n\n(--quit) to quit");
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input = input.trim();
        if input == "--quit" {
            break;
        }
        let input = match input.parse() {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Expected 1 or 2 but got {} instead!?", input);
                break;
            }
        };
        match input {
            1 => {
                auto_sample_pass::main()
            }
            2 => {
                self_sampled::main()
            }
            _ => {
                eprintln!("Expected 1 or 2 but got {} instead!?",input);
                break;
            }
        }

    }
    
}