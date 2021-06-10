// cargo run S Cargo.toml
// CASE_INSENSITIVE=1 cargo run S Cargo.toml


/*
Rust 的运行速度、安全性、单二进制文件输出和跨平台支持使其成为创建命令行程序的绝佳选择，所以我们的项目将创建一个我们自己版本的经典命令行工具：grep。
grep 是 “Globally search a Regular Expression and Print.” 的首字母缩写。
grep 最简单的使用场景是在特定文件中搜索指定字符串。
为此，grep 获取一个文件名和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。

在这个过程中，我们会展示如何让我们的命令行工具利用很多命令行工具中用到的终端功能。读取环境变量来使得用户可以配置工具的行为。
打印到标准错误控制流（stderr） 而不是标准输出（stdout），例如这样用户可以选择将成功输出重定向到文件中的同时仍然在屏幕上显示错误信息。


// ========================================================================
// 二进制项目的关注分离
// ========================================================================
main 函数负责多个任务的组织问题在许多二进制项目中很常见。所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导性过程。
这些过程有如下步骤：
    将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。
    当命令行解析逻辑比较小时，可以保留在 main.rs 中。
    当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。

经过这些过程之后保留在 main 函数中的责任应该被限制为：
    使用参数值调用命令行解析逻辑
    设置任何其他的配置
    调用 lib.rs 中的 run 函数
    如果 run 返回错误，则处理这个错误

这个模式的一切就是为了关注分离：main.rs 处理程序运行，而 lib.rs 处理所有的真正的任务逻辑。
因为不能直接测试 main 函数，这个结构通过将所有的程序逻辑移动到 lib.rs 的函数中使得我们可以测试他们。
仅仅保留在 main.rs 中的代码将足够小以便阅读就可以验证其正确性。
*/

use std::env;
use std::process;

use minigrep::Config;

fn main() {

    // ========================================================================
    // 读取参数值
    // ========================================================================
    /*
    获取命令行参数的值: std::env::args返回一个传递给程序的命令行参数的 迭代器（iterator）
    在迭代器上调用 collect 方法将其转换为一个集合，比如包含所有迭代器产生元素的 vector。

    args 函数和无效的 Unicode
    注意 std::env::args 在其任何参数包含无效 Unicode 字符时会 panic。
    如果你需要接受包含无效 Unicode 字符的参数，使用 std::env::args_os 代替。这个函数返回 OsString 值而不是 String 值。

    这里出于简单考虑使用了 std::env::args，因为 OsString 值每个平台都不一样而且比 String 值处理起来更为复杂。
    */
    println!("运行参数： {:?}", env::args());
    // 直接使用 env::args 返回的迭代器: 一旦 Config::new 获取了迭代器的所有权并不再使用借用的索引操作，
    // 就可以将迭代器中的 String 值移动到 Config 中，而不是调用 clone 分配新的空间。
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        // println! 函数只能够打印到标准输出，标准库提供了 eprintln! 宏来打印到标准错误流
        // cargo run > out.txt 如此依旧能看到错误打印
        eprintln!("Problem parsing arguments: {}", err);
        // 非零的退出状态是一个惯例信号，用来告诉调用程序的进程：该程序以错误状态退出了
        process::exit(1);   // std::process::exit 会立即停止程序并将传递给它的数字作为退出状态码。
    });

    // println!("Searching for {}", cfg.query);
    // println!("In file {}", cfg.filename);

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application error: {}", e);
        // 非零的退出状态是一个惯例信号，用来告诉调用程序的进程：该程序以错误状态退出了
        process::exit(1);   // std::process::exit 会立即停止程序并将传递给它的数字作为退出状态码。
    }
}
