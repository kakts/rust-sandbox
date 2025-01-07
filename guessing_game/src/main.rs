use std::io;

fn main() {
    println!("Hello, world!");

    println!("Please input your guess.");

    let mut guess = String::new();

    // stdinからの入力を受け取る
    // Result.expect()は、ResultがErrの場合に引数のメッセージを表示してプログラムをクラッシュさせる
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
