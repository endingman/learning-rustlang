fn main() {
    /*
       // 只能存在一个可变引用
       let mut s = String::from("Hello");
       let mut ss = &s;
       let mut sss = &s;
       /*
       |
       |     let mut sss = &s;
       |         ----^^^
       |         |
       |         help: remove this `mut`
       */

       // 可存在多个不可变引用
       let a=String::from("Hello");
       let aa=&a;
       let aaa=&a;
    */

    /*
     我们必须将 String 返回给调用函数，以便在调用 calculate_length 后仍能使用 String，
     因为 String 被移动到了 calculate_length 内。

    下面是如何定义并使用一个（新的）calculate_length 函数，
    它以一个对象的引用作为参数而不是获取值的所有权

    &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。
    因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。
     */
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
// & 符号就是 引用，它们允许你使用值但不获取其所有权。
// 使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*
fn calculate_length(s: &String) -> usize {
    // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

// 我们将获取引用作为函数参数称为 借用（borrowing）,没有所有权并不能修改值
// 正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。
/*
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
 */
