use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn main() {
    let string : String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(14)
        .map(char::from)
        .collect();
    println!("Generated Psuedorandom password: {}", string);
}
