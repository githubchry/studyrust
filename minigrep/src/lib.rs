// 注释文档生成：注释包含项的结构: //! => 这为包含注释的项，而不是位于注释之后的项增加文档。
// 这通常用于 crate 根文件（通常是 src/lib.rs）或模块的根文件为 crate 或模块整体提供文档。
// cargo doc --open    => 生成文档：minigrep/target/doc/minigrep/index.html
// 以下注释显示在 minigrep 文档的首页，位于 crate 中公有项列表之上

//! # mini grep
//!
//! `minigrep` 获取一个文件名和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。


use std::error::Error;
use std::fs;
use std::env;

// ======== 以下代码段与项目无关，仅用于示范文档注释生成 ========
// 注释文档生成：使用 pub use 导出合适的公有 API
// cargo doc --open    => 生成文档：minigrep/target/doc/minigrep/index.html => Re-exports
// pub use才会生成Re-exports文档段，仅use不行...
pub use self::kinds::PrimaryColor;
pub use self::utils::mix;


// cargo doc --open    => 生成文档：minigrep/target/doc/minigrep/index.html => Modules => kinds
pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

// cargo doc --open    => 生成文档：minigrep/target/doc/minigrep/index.html => Modules => utils
pub mod utils {
    use crate::kinds::*;

    // 下面是markdown格式文档注释，所以在web显示时会续成一行...
    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
// ======== 以上代码段与项目无关，仅用于示范文档注释生成 ========

pub struct Config {
    // 要搜索的字符串
    pub query: String,
    // 要搜索的文件名
    pub filename: String,

    /*
    增加一个额外的功能来改进 minigrep： 用户可以通过设置`环境变量`来设置搜索是否是大小写敏感的 。
    当然，我们也可以将其设计为一个命令行参数并要求用户每次需要时都加上它，不过在这里我们将使用环境变量。
    这允许用户设置环境变量一次之后在整个终端会话中所有的搜索都将是大小写不敏感的。

    设置环境变量方法：
    PowerShell  => $env:CASE_INSENSITIVE=1  查看 $env:CASE_INSENSITIVE
    cmd         => set CASE_INSENSITIVE=1   查看 set CASE_INSENSITIVE
    linux       => CASE_INSENSITIVE=1       查看 echo $CASE_INSENSITIVE
    */
    pub case_sensitive: bool, // 大小写敏感
}

impl Config {
    /*
    注释文档生成：文档注释使用三斜杠 /// 而不是两斜杆以支持 Markdown 注解来格式化文本。文档注释就位于需要文档的项的之前。
    常用的文档注释： Examples Panics Errors Safety
    在文档注释中增加示例代码块是一个清楚的表明如何使用库的方法，这么做还有一个额外的好处：cargo test 也会像测试那样运行文档中的示例代码！
    需要注意：写完文档后改动了代码，会导致例子不能正常工作。

    cargo doc --open    =>生成文档：minigrep/target/doc/minigrep/index.html => struct.Config.html
    cargo test  => 单元测试unittests和文档测试Doc-tests
    */

    /// 根据运行参数，创建并返回一个`Config`结构体
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = chry_minigrep::Config::new(std::env::args());
    /// /*
    /// let cfg = chry_minigrep::Config::new(std::env::args()).unwrap_or_else(|err| {
    ///    eprintln!("Problem parsing arguments: {}", err);
    ///    process::exit(1);
    /// });
    /// */
    /// ```

    // 因为我们拥有 args 的所有权，并且将通过对其进行迭代来改变 args ，所以我们可以将 mut 关键字添加到 args 参数的规范中以使其可变。
    // 返回 Result 而不应该 返回Config或调用 panic!
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // &'static str => 显式静态声明 => 等价于隐式静态声明 &str
        args.next();    // env::args 返回值的第一个值是程序的名称, 忽略并获取下一个值

        // 要搜索的字符串
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // 要搜索的文件名
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };

        // println!("case_sensitive: {:?}", env::var("CASE_INSENSITIVE"));
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // 注意这里仅判断环境变量存在与否，不关心环境变量的内容！
        // println!("case_sensitive: {}", case_sensitive);
        /*
        env::var 返回一个 Result，它在环境变量被设置时返回包含其值的 Ok 成员，并在环境变量未被设置时返回 Err 成员。
        使用 Result 的 is_err 方法来检查其是否是一个 error（也就是环境变量未被设置的情况），这也就意味着我们 需要 进行一个大小写敏感搜索。
        如果CASE_INSENSITIVE 环境变量被设置为任何值，is_err 会返回 false 并将进行大小写不敏感搜索。
        我们并不关心环境变量所设置的 值，只关心它是否被设置了，所以检查 is_err 而不是 unwrap、expect 或任何我们已经见过的 Result 的方法。
        */

        Ok(Config { query, filename, case_sensitive })
    }
}

/*
测试驱动开发（Test Driven Development, TDD）的模式来逐步增加 minigrep 的搜索逻辑。这是一个软件开发技术，它遵循如下步骤：
    1.编写一个失败的测试，并运行它以确保它失败的原因是你所期望的。
    2.编写或修改足够的代码来使新的测试通过。
    3.重构刚刚增加或修改的代码，并确保测试仍然能通过。
    4.从步骤 1 开始重复！
这只是众多编写软件的方法之一，不过 TDD 有助于驱动代码的设计。在编写能使测试通过的代码之前编写测试有助于在开发过程中保持高测试覆盖率。

编写测试函数模块，就可去掉 src/lib.rs 和 src/main.rs 中用于检查程序行为的 println! 语句，因为不再真正需要他们了。
*/
// ========================================================================
// 编写失败的测试
// ========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "hello";
        let contents = "\
01 abcdefg
    02 hello world
03 Hello world
04 123465789";

        assert_eq!(vec!["    02 hello world"], search(query, contents));
    }


    // 编写一个大小写不敏感 search 函数的失败测试
    #[test]
    fn case_insensitive() {
        let query = "hello";
        let contents = "\
01 abcdefg
    02 hello world
03 Hello world
04 123465789";

        assert_eq!(vec!["    02 hello world", "03 Hello world"], search_case_insensitive(query, contents));
    }
}

// ========================================================================
// 编写使测试通过的代码
// ========================================================================
/*
search函数设计：
    遍历内容的每一行文本。
    查看这一行是否包含要搜索的字符串。
    如果有，将这一行加入列表返回值中。
    如果没有，什么也不做。
    返回匹配到的结果列表

使用显式生命周期'a：表明contents的生命周期和返回的vector生命周期相关联
因为实现里面vector包含了contents slice的字符串 slice
*/
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
    let mut results = Vec::new();
    // 使用 lines 方法遍历每一行
    for line in contents.lines() {
        // 用查询字符串搜索每一行
        if line.contains(query) {
            // 存储匹配的行
            results.push(line);
        }
    }
    results
    */
    // 使用迭代器适配器来使代码更简明 也避免了一个可变的中间 results vector 的使用。
    // 函数式编程风格倾向于最小化可变状态的数量来使代码更简洁。
    // 去掉可变状态可能会使得将来进行并行搜索的增强变得更容易，因为我们不必管理 results vector 的并发访问。
    contents.lines()
        .filter(|line| line.contains(query))    // 使用 filter 适配器只保留 line.contains(query) 返回 true 的那些行
        .collect()                              // 将匹配行收集到另一个 vector 中
}


fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    // 使用 lines 方法遍历每一行
    for line in contents.lines() {
        // 用查询字符串搜索每一行
        if line.to_lowercase().contains(&query) {
            // 存储匹配的行
            results.push(line);
        }
    }
    results
}


pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型。
    // 这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。这也就是 dyn，它是 “动态的”（“dynamic”）的缩写。
    // 使用 ? => 允许返回的 “任何类型的错误(实现了Error trait的类型)” => Box<dyn Error>
    // 可以后头看一下17result.rs中的传播（propagating）概念
    let contents = fs::read_to_string(cfg.filename)?;
    // println!("With text:{}", contents);
    // println!("Hello, world!");

    let results = if cfg.case_sensitive {
        search(&cfg.query, &contents)
    } else {
        search_case_insensitive(&cfg.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}