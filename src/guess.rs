use std::io;

// mod guess;
pub fn guess_name(){
    println!("Guess the number!");
    println!("Please input your guess");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guessed: {guess}");
}
