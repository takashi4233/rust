struct User {
	 username:String,
	 email:String,
	 sign_in_count:u64,
	 active:bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//省略記法
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");
	 let mut user1 = User {
		  email: String::from("someone@example.com"),
		  username: String::from("someusername123"),
		  active: true,
		  sign_in_count: 1,
	 };

	 println!("{}",user1.email);
	 user1.email = String::from("anotheremail@example.com");
	 println!("{}",user1.email);

	 let user2 = build_user(String::from("hoge@gmail.com"),String::from("Hoge Page"));
	 println!("{}",user2.email);

	 let user3 = build_user(String::from("page@gmail.com"),String::from("Pa Pa Pa"));
	 println!("{}",user3.email);

	 //構造体更新法記
	 // ..Structで変更が不要な値はそのままコピーできる
	 let user4 = User {
		  email:String::from("aaa@gmail.com"),
		  ..user1
	 };
	 println!("{},{}",user4.username,user4.email);
}
