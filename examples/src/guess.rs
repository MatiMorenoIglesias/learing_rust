use std::io;

// mod guess;
pub fn guess_name(){
    println!("Guess the number!");
    println!("Please input your guess");
    let mut guess = String::new();

    let results = io::stdin().read_line(&mut guess);
    match results {
        Ok(val) => {
            // Done
            println!("Your guessed: {guess} and the length is {},", guess.len());
        },
        Err(val) => {
            // Failed
            println!("[Error] {}.", val);
        }
    }
    //    .expect("Failed to read line");

}

