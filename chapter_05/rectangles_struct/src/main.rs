struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
// 函数 area 现在被定义为接收一个名叫 rectangle 的参数，
// 其类型是一个结构体 Rectangle 实例的不可变借用。
// 第四章讲到过，我们希望借用结构体而不是获取它的所有权，
// 这样 main 函数就可以保持 rect1 的所有权并继续使用它，
// 所以这就是为什么在函数签名和调用的地方会有 &。

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
