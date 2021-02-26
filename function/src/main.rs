Fnca main() {
    println!("Hello, world!");
    another_function(5);
    let _x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("Five is : {}", five());
    println!("The value of y is : {}", y);
}

// 引数の型宣言は必須
fn another_function(x: i32) {
    println!("Another function : {}", x);
}

fn five() -> i32 {
    3 + 2
}
