fn main() {
	 let s1 = gives_ownership();
	 println!("{}",s1);

	 let s2 = String::from("Hello");
	 let s3 = takes_and_gives_back(s2);
	 println!("{}",s3);
	 
	 let s = String::from("hello");
	 takes_ownership(s);
 
	 //エラーになる
	 //println!("{}",s);

	 let x = 5;
	 makes_copy(x);
	 println!("{}",x);
}


fn takes_and_gives_back(a_string:String) -> String {
	 a_string
}

fn gives_ownership() -> String {
	 let some_string = String::from("Hello");
	 some_string
}

fn takes_ownership(some_string:String){
	 println!("{}",some_string);
}

fn makes_copy(some_integer:i32) {
	 println!("{}",some_integer);
}
