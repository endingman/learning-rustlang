mod sound {
    pub mod instrument {
        // pub 关键字使项变为公有
        // 私有性规则适用于结构体、枚举、函数和方法以及模块。
        pub fn clarinet() {
            // 函数体
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet(); //pub公有项才可以调用

    // Relative path
    sound::instrument::clarinet();
}
