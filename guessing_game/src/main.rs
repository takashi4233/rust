use std::io;

fn main() {
    println!("Guess the number!");
	 println!("Please input your guess.");
	 let mut guess = String::new();	//Rustでは、変数は標準で不変。mutをつけて可変にする。

	 io::stdin().read_line(&mut guess).expect("Failed to read line");

	 println!("Your guessed:{}",guess);
	 
	 
}
