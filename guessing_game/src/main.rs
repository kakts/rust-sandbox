use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let rand_num = get_random_num();
    let mut count = 0;
    loop {
        // stdinからの入力を受け取る
        // Result.expect()は、ResultがErrの場合に引数のメッセージを表示してプログラムをクラッシュさせる
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);
        let guess_num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Please input a valid number! {}", e);
                guess.clear();
                continue;
            }
        };

        match guess_num.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! challenge count is : {}", &count),            
                break;
            }
        }
        guess.clear();
        count += 1;
    }


}


fn get_random_num() -> i32 {

    let mut rng = rand::thread_rng();
    return rng.gen_range(1..100);
}
