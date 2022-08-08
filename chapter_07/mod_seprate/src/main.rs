/**
在 mod sound 后使用分号而不是代码块告诉 Rust 在另一个与模块同名的文件中加载模块的内容。
*/
mod sound;

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet(); //pub公有项才可以调用

    // Relative path
    sound::instrument::clarinet();
}
