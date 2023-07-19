use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() 
{
    println!("Welcome to the guessing game!\n");

    let secret_number:u32 = rand::thread_rng().gen_range(1..=100);
    let mut attempts:u8 = 0;

    'game_loop: loop 
    {
    println!("Please enter your number!");
    let mut guess:String = String::new();

    io::stdin ()
        .read_line(&mut guess)
        .expect("Failed to read number!");
    
    let guessed_number: u32 = match guess.trim().parse::<u32>() 
    {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed {}", guessed_number);

    match guessed_number.cmp(&secret_number)
    {
        Ordering::Less => { 
            println!("{}", "Too small!".red());
            attempts+=1;
            println!("Your attempt: {}", attempts);
        },
        Ordering::Equal => { 
            println!("{}", "You won!".green());
            attempts+=1;
            println!("Your attempt: {}", attempts);
            if attempts < 7 {
                println!("{}", "Very good job!".green())
            } else if attempts == 7 {
                println!("{}", "Good job!".yellow());
            } else {
                println!("{}", "You can do better".red());
            }

            break 'game_loop;
        },
        Ordering::Greater => {
            println!("{}", "Too big!".red());
            attempts+=1;
            println!("Your attempt: {}", attempts);
        },
    };
    
    }
}
