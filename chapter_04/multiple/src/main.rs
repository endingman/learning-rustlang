fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

/*
*在每一个函数中都获取所有权并接着返回所有权有些啰嗦。
*如果我们想要函数使用一个值但不获取所有权该怎么办呢？
*如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，除此之外，我们也可能想返回函数体中产生的一些数据。
*我们可以使用元组来返回多个值，如示例 4-5 所示。
*/
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
