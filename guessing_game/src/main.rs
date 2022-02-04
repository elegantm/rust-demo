use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed read line");

        println!("You guessed: {}", guess);
    
        let parse_result = guess.trim().parse();
        let guess: u32 = match parse_result {
            // If parse succeeded evaluate to the number
            Ok(num) => num,
            // If parse failed repeat the loop
            Err(msg) => {
                println!("input error : {}", msg);
                continue;
            },
        };

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
