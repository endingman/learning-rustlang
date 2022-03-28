#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 每个结构体都允许拥有多个 impl 块。
// 例如，示例 5-16 中的代码等同于示例 5-15，但每个方法有其自己的 impl 块
// 这里没有理由将这些方法分散在多个 impl 块中，不过这是有效的语法。
// 第十章讨论泛型和 trait 时会看到实用的多 impl 块的用例。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
    /*
        带有更多参数的方法#
        让我们通过实现 Rectangle 结构体上的另一方法来练习使用方法。
        这回，我们让一个 Rectangle 的实例获取另一个 Rectangle 实例，
        如果 self 能完全包含第二个长方形则返回 true；
    */
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 area is  ? {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
