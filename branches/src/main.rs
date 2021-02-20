fn main() {
    // 1 - 3までを逆順に出力する
    for number in (1..4).rev() {
        println!("{}", number);
    }

    // while よりも for を使う
    let ary = [10, 20, 30, 40, 50];
    for element in ary.iter() {
        println!("the value is : {}", element);
    }
    // iter()で取り出して、rev()で逆順にする
    for element in ary.iter().rev() {
        println!("the value is : {}", element);
    }

    let condition = true;
    //if式で値を作ることも可能
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);

    let number = 3;

    // Rustのif分は必ずbool型である必要あり。
    // Pythonでは、0はFalse, ""はFalseなどになっている。
    // -> Python側のプロジェクトで動きを復習したい
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
