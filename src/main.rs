#![allow(non_snake_case)]
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    control::set_virtual_terminal(true).unwrap();
    let randomnumber = rand::thread_rng().gen_range(1..51);
    let mut attemptcount = -0;
loop {
        println!("Guess a number 1-50");

        let mut uinp = String::new();

        io::stdin()
        .read_line(&mut uinp)
        .expect("failed");
        let uinp: u32 = match uinp.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("you guessed: {}", uinp);
        attemptcount += 1;
        match uinp.cmp(&randomnumber){
            Ordering::Less => println!("Goal {}", "Higher".yellow().italic()),
            Ordering::Greater => println!("Goal {}", "Lower".yellow().italic()),
            Ordering::Equal => {
                print!("{}[2J", 27 as char);
                let attempttostring = attemptcount.to_string();
                println!("{} it took |{}| attepts", "You Win!".green().bold().underline(), attempttostring.blue());
                break;
            }
        }
    }
}