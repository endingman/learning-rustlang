fn main() {
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

// fn dangle() -> &String {
//     // dangle 返回一个字符串的引用

//     let s = String::from("hello"); // s 是一个新字符串

//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
//   // 危险！

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
