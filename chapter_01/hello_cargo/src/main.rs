fn main() {
    println!("Hello, world!");
    another_function();

    let x = 5;
    println!("The value of x is: {}", x);

    /*
     *表达式的结尾没有分号。
     *如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
     */
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn another_function() {
    println!("Another function");
}
