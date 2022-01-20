use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess");
        /*
         * mut 可变变量，可以重复赋值
         * 没有 mut 变量不可变，不可以重复赋值
         */
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // 这里创建了一个叫做 guess 的变量。
        // 不过等等，不是已经有了一个叫做 guess 的变量了吗？
        // 确实如此，不过 Rust 允许用一个新值来 隐藏 （shadow） guess 之前的值。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => println!("You Win"),
            Ordering::Less => println!("Too small"),
            Ordering::Greater => {
                println!("Too big");
                break;
            }
        }
    }
}
