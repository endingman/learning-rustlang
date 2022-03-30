#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // 绑定值的模式
    // 匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值。
    // 可以将这些信息加入我们的 enum，通过改变 Quarter 成员来包含一个 State 值
    Quarter(UsState),
}

// 一个枚举和一个以枚举成员作为模式的 match 表达式
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        // 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alabama));
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

/*
_ 通配符#
Rust 也提供了一个模式用于不想列举出所有可能值的场景。例如，u8 可以拥有 0 到 255 的有效的值，如果我们只关心 1、3、5 和 7 这几个值，就并不想必须列出 0、2、4、6、8、9 一直到 255 的值。所幸我们不必这么做：可以使用特殊的模式 _ 替代：

let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
*/
