extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	let secret_number = rand::thread_rng().gen_range(1,101);
	
    println!("The secret numbre is:{}",secret_number);
	 println!("Please input your guess.");
	 let mut guess = String::new();	//Rustでは、変数は標準で不変。mutをつけて可変にする。

	 io::stdin().read_line(&mut guess).expect("Failed to read line");

	 let guess: u32 = guess.trim().parse().expect("Please type a number!");

	 println!("Your guessed:{}",guess);
	 match guess.cmp(&secret_number) {
	 		 Ordering::Less => println!("Too small!"),
			 Ordering::Greater => println!("Too big!"),
			 Ordering::Equal=>println!("You win!"),
	}
	 
}
