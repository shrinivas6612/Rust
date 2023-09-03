use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number: ");
    
    let rand_number=rand::thread_rng().gen_range(1..=100);
    loop{

        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Error in the input try again");

        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        match guess.cmp(&rand_number){
            Ordering::Less=>println!("Too Small"),
            Ordering::Greater=>println!("Too big"),
            Ordering::Equal=>{
                println!("yee win");
                break;
            }
        }

    };
}
