pub mod test;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Enter guess");
    let secret = rand::thread_rng().gen_range(1,101);
    println!("Secret is {}",secret);
    loop {
    let mut guess  = String ::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    let guess:u32 = match guess.trim().parse(){
        Ok(num) =>num,
        Err(_)=>{
            println!("Enter a number");
            continue;
        },
    };
        println!("You guessed : {}",guess);
    
    
    match guess.cmp(&secret){
        Ordering::Equal =>{ println!("U win");
    break;},
        Ordering::Greater =>println!("Too big!"),
        Ordering::Less =>println!("Too small"),
    }
    }
    
    
}
