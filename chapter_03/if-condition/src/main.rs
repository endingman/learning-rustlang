fn main() {
    let condition = true;
    let number = if condition {
        5 //数字本身是分表达式
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
