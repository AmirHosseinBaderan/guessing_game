use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // select a random number as result
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count_of_try = 0;

    loop {
        count_of_try+=1;
        println!("Please input guess.");
        // declare the guess from user and get input
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You guessed : {guess}");

        let guess: u32 = guess.trim().parse().expect("Please input a valid number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win,with {count_of_try} try");
                break;
            }
        }
    }
}
