use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guess(){

    let secret = rand::thread_rng().gen_range(1, 101);
    
    loop{
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Error reading from console");
        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Nice work! You Win!");
                break;
            }
        }
    }
}

pub fn run(){
    println!("Guess the number!");
    guess();
}