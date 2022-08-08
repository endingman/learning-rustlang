mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // 函数体
        }
    }
}
/**
 * 当使用 use 关键字将名称导入作用域时，在新作用域中可用的名称是私有的。
 * 如果希望调用你编写的代码的代码能够像你一样在其自己的作用域内引用这些类型，可以结合 pub 和 use。
 * 这个技术被称为 「重导出」（re-exporting），因为这样做将项引入作用域并同时使其可供其他代码引入自己的作用域。
 */
mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}
/**
通过 glob 运算符将所有的公有定义引入作用域
如果希望将一个路径下 所有 公有项引入作用域，可以指定路径后跟 *，glob 运算符：
use std::collections::*;
*/

/**
 使用外部包：
为了在项目中使用 rand，在 Cargo.toml 中加入了如下行：
文件名: Cargo.toml
[dependencies]
rand = "0.5.5"
 */

/**
注意标准库（std）对于你的包来说也是外部 crate。
因为标准库随 Rust 语言一同分发，无需修改 Cargo.toml 来引入 std，
不过需要通过 use 将标准库中定义的项引入项目包的作用域中来引用它们，比如 HashMap：
use std::collections::HashMap;
*/
fn main() {
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
}
