fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

// 表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
fn five() -> i32 {
    5
}
