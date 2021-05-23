// 项目隐式地链接了标准库 可通过添加no_std属性来禁用它:
// #![no_std]   //但是因为println宏是标准库的一部分，所以会发生编译错误

// 编译: rustc main.rs

fn main() {
    // Rust 的缩进风格使用 4 个空格，而不是 1 个制表符（tab）。
    // 当看到符号 ! 的时候，就意味着调用的是宏而不是普通函数
    println!("Hello, world!");
}
