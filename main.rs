use std::io;
use rand; // 0.9.0

fn main(){
    // Simple number guessing game
    // - project to say Hello to Rust...
    
    let secret_number: i32 = rand::random_range(0..=500);
    
    let mut is_correct: i32 = 0;
    
    while is_correct == 0{
        let mut guessed_string = String::new();
        println!("Enter a number!");
        
        io::stdin()
            .read_line(&mut guessed_string)
            .expect("Failed to read line");
        
        let guessed_string = guessed_string.trim_end();
        
        println!("You guessed: {}",guessed_string);
        
        let int_guess: i32 = guessed_string.parse()
                            .expect(" Not a valid number!");
    
        if int_guess > secret_number{
            println!("Secret number is lower!\n");
        }
        else if int_guess < secret_number{
            println!("Secret number is higher!\n");
        }
        else{
            println!("Correct!");
            is_correct = 1;
        }
    }
}
