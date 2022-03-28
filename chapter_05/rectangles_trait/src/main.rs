// 增加注解来派生 Debug trait，并使用调试格式打印 Rectangle 实例
#[derive(Debug)]

/*
println! 宏能处理很多类型的格式，
不过，{} 默认告诉 println! 使用被称为 Display 的格式：
意在提供给直接终端用户查看的输出。目前为止见过的基本类型都默认实现了 Display，
因为它就是向用户展示 1 或其他任何基本类型的唯一方式。不过对于结构体，println!
应该用来输出的格式是不明确的，因为这有更多显示的可能性：是否需要逗号？需要打印出大括号吗？所有字段都应该显示吗？
由于这种不确定性，Rust 不会尝试猜测我们的意图，
所以结构体并没有提供一个 Display 实现。
*/
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("width is {}", rect1.width); //{:#?}更好看的打印格式
    println!("height is {}", rect1.height); //{:#?}更好看的打印格式
    println!("rect1 is {:#?}", rect1); //{:#?}更好看的打印格式
}
