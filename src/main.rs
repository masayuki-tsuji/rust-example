use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("予想を入力してみてね！");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込み失敗");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数字を入力してください");
                continue;
            }
        };

        println!("あなたの数字: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ!"),
            Ordering::Greater => println!("大きすぎ!"),
            Ordering::Equal => {
                println!("正解");
                break;
            }
        }
    }
}
