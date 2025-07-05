use std::cmp::Ordering;
use std::io;
// use rand::Rng;

fn main() {
    println!("Guess The Number App\n--------------------");

    // let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number = 12;

    loop {
        println!("\nPlease input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your guess is not a number.");
                continue;
            }
        };
        println!("You guessed: {guess}",);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
