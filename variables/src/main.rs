fn main() {

	 let mut c = "hoge!";
	 println! ("word:{}",c);
	 c = "page!";
	 println! ("word:{}",c);

	 
	 let mut s = String::from("hello");
	 println! ("word:{}",s);
	 s.push_str(", world!");
	 println! ("word:{}",s);

	 
	 const MAX_POINTS: u32 = 100_000;
	 println!("MAX_POINTS is:{}",MAX_POINTS);
	 let mut x = 5;
	 println!("The value of x is:{}",x);
	 x = 6;
	 println!("The value of x is:{}",x);

	 // tapple
	 let tap = (1.0,"Apple",'A');
	 println!("tap.1 is {}",tap.1);
	 let (_x,_y,z) = tap;
	 println!("z is :{}",z);
	 
	 // array
	 let ary = ["apple","orange","grape"];
	 println!("ary[1] is {}",ary[1]);
	

}
