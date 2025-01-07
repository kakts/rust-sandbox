use std::io;
use rand::Rng;

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

        if guess_num == rand_num {
            println!("You win! challenge count is : {}", &count);
            break;
        } else if guess_num > rand_num{
            println!("{} is too big!", &guess_num);
        } else {
            println!("{} is too small!",&guess_num);
        }
        guess.clear();
        count += 1;
    }


}


fn get_random_num() -> i32 {

    let mut rng = rand::thread_rng();
    return rng.gen_range(1..100);
}
