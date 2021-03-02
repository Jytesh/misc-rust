use bcrypt::{DEFAULT_COST, Version};
use ring::rand::SecureRandom;
use ring::rand;
use progress::SpinningCircle;
use std::sync;

struct Spinner{
    sender: sync::mpsc::Sender<String>,
}
impl Spinner {
    fn new(title: String) -> Spinner{
        let mut spinner = SpinningCircle::new();
        let (sender, receiver) = sync::mpsc::channel::<String>();
        spinner.set_job_title(&title[..]);
        std::thread::spawn( move || loop {
            match receiver.try_recv() {
                Ok(x) => {
                    if &x[..] == "Done" {
                        break;
                    } else if x.starts_with("Change ") {
                        let x  = &x[..].replace("Change ","");
                        spinner.set_job_title(x);
                    }
                }
                _ => {
                    spinner.tick();
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    continue;
                }
            }
        });

        Spinner {
            sender
        }
    }
    fn stop(&self) {
        self.sender.send(String::from("Done")).unwrap();
    }
    fn change_title(&self, title : String) -> Result<(), sync::mpsc::SendError<String>> {
        self.sender.send(format!("Change {}", title))
    }
}

pub fn main(args: Vec<String>) {
    let spinner = Spinner::new(String::from("Generating Salts"));
    let password = args[2..].join(" ");
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt).unwrap();
    spinner.change_title(String::from("Generating Hash")).unwrap();
    println!("\n\nHashed: {}", bcrypt::hash_with_salt(password, DEFAULT_COST, &salt).unwrap().format_for_version(Version::TwoA));
    
    spinner.stop();
}