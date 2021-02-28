use std::io;

#[derive(Debug)]
struct Chars {
    a : usize,
    c : usize,
    g : usize,
    t : usize,
}
impl Chars {
    fn new() -> Chars {
        Chars {
            a: 0,
            c: 0,
            g: 0,
            t: 0
        }
    }
}
fn main() {
    println!("Welcome to DNA Parser, enter your DNA Sequence!: ");
    let mut input = String::new();
    let mut chars = Chars::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    for ch in input.to_ascii_lowercase().trim().chars() {
        match ch {
            'a' => {
                chars.a += 1
            },
            'c' => {
                chars.c += 1
            },
            'g' => {
                chars.g += 1
            },
            't' => {
                chars.t += 1
            }
            _ => {
                eprint!("INVALID CHARACTER ENCOUNTERED!")
            }
        }
    }
    println!("{:?}", chars)
}
