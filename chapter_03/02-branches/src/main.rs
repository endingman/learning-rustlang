fn main() {
    let number = 6;
    //  Rust 只会执行第一个条件为真的代码块，并且一旦它找到一个以后，甚至都不会检查剩下的条件了。
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
