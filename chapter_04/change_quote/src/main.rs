/*
首先，必须将 s 改为 mut。然后必须创建一个可变引用 &mut s 和接受一个可变引用 some_string: &mut String。

不过可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。这些代码会失败：
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);


这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它可由这三个行为造成：

    两个或更多指针同时访问同一数据。
    至少有一个指针被用来写入数据。
    没有同步数据访问的机制。
*/
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
