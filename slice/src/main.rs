fn main() {
	 let mut s = String::from("hoge1 hoge2 hoge3");
	 let word = first_word(&s);
    println!("word:{},s:{}",word,s);

	 let hello = &s[0..5];
	 println!("{}",hello);


	 s.clear();
	 println!("word:{},s:{}",word,s);


}


fn first_word(s:&String) -> usize {
	 let bytes = s.as_bytes();

	 for (i,&item) in bytes.iter().enumerate() {
		  if item == b' ' {
				return i;
		  }
	 }

	 s.len()
}
