use std::env;
use ansi_term::Color::{Red, Green, Blue, White};

mod tldr;

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect(); 

    args.remove(0);

    let tldr = tldr::tldr {
        search: args[0].clone() 
    };

    let info = tldr.get_tldr().await;

    for line in info.split("\n") {
        if line.chars().next() == Some('#') {
            println!("{}", Red.bold().paint(line.to_string().replace("#", ""))) 
        }

        if line.chars().next() == Some('>') {
            println!("{}", Blue.paint(line))
        }

        if line.chars().next() == Some('-') {
            println!("{}", Green.paint(line.to_string().replace("`", "")))
        }

        if line.chars().next() == Some('`') {
            println!("{}", White.paint(line.to_string().replace("`", "")))
        }
    }
}
