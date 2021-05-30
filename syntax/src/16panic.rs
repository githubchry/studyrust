/*
当出现 panic 时，程序默认会开始 展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。
另一种选择是直接 终止（abort），这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理。
如果你需要项目的最终二进制文件越小越好，panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止。
例如，如果你想要在release模式中 panic 时直接终止：

[profile.release]
panic = 'abort'

*/
fn main() {
    /*
    cargo run --bin 16panic
       Compiling studyrust v0.1.0 (/mnt/d/codes/git/studyrust/grammar)
        Finished dev [unoptimized + debuginfo] target(s) in 2.23s
         Running `target/debug/16panic`
    thread 'main' panicked at 'crash and burn', src/16panic.rs:20:5
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    最后两行包含 panic! 调用造成的错误信息。
    第一行显示了 panic 提供的信息并指明了源码中 panic 出现的位置
    第二行提示我们可以设置 RUST_BACKTRACE 环境变量来得到一个 backtrace
    */
    // panic!("crash and burn");

    // ========================================================================
    // 使用 panic! 的 backtrace
    // ========================================================================
    /*
    backtrace 是一个执行到目前位置所有被调用的函数的列表。Rust 的 backtrace 跟其他语言中的一样：
        阅读 backtrace 的关键是从头开始读直到发现你编写的文件。
    这就是问题的发源地。这一行往上是你的代码所调用的代码；往下则是调用你的代码的代码。
    这些行可能包含核心 Rust 代码，标准库代码或用到的 crate 代码。

    为了获取带有这些信息的 backtrace，必须启用 debug 标识。
    当不使用 --release 参数运行 cargo build 或 cargo run 时 debug 标识会默认启用。

    模拟一个 缓冲区溢出 代码进行测试：
    */
    let v = vec![1, 2, 3];

    v[99];

    /*
    运行参数：RUST_BACKTRACE=1 cargo run --bin 16panic
    可以看到大量输出！
    */
}