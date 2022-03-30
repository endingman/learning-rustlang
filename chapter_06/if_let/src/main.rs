#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // if let 简单控制流#
    // if let 语法让我们以一种不那么冗长的方式结合 if 和 let，来处理只匹配一个模式的值而忽略其他模式的情况。
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("count quarter from is: {}", count);
    }

    // match 模式
    let coin = Coin::Nickel;
    count = 1;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        // _ 通配符#
        // Rust 也提供了一个模式用于不想列举出所有可能值的场景。
        _ => count += 1,
    }

    println!("count quarter from is: {}", count);
}
