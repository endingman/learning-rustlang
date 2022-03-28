#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
impl 块的另一个有用的功能是：允许在 impl 块中定义 不 以 self 作为参数的函数。
这被称为 关联函数（associated functions），因为它们与结构体相关联。
它们仍是函数而不是方法，因为它们并不作用于一个结构体的实例。
你已经使用过 String::from 关联函数了。

关联函数经常被用作返回一个结构体新实例的构造函数。
例如我们可以提供一个关联函数，它接受一个维度参数并且同时作为宽和高，
这样可以更轻松的创建一个正方形 Rectangle 而不必指定两次同样的值：
*/
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 使用结构体名和 :: 语法来调用这个关联函数
    let sq = Rectangle::square(3);
    println!("width size  is ? {}", sq.width);
    println!("height size is ? {}", sq.height);
    println!("square is ? {:#?}", sq);
}
