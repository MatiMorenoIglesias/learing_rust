use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    loop {
        println!("input number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Caution: The guess value must be number type...");
                continue
            },
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!, try again!"),
            Ordering::Greater => println!("Too big!, try again!"),
            Ordering::Equal => {println!("You win!"); break},
        }
    }
}
