use std::error::Error;
use std::fs;
use std::env;


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
    // 注意入参类型等价: Vec<String>与[String]
    // 返回 Result 而不应该 返回Config或调用 panic!
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // &'static str => 显式静态声明 => 等价于隐式静态声明 &str
        // 必要的错误处理
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();       // 要搜索的字符串
        let filename = args[2].clone();    // 要搜索的文件名
        // ========================================================================
        // 使用 clone 的权衡取舍
        // ========================================================================
        /*
        最简单但有些不太高效的方式是调用这些值的 clone 方法。
        这会生成 Config 实例可以拥有的数据的完整拷贝，不过会比储存字符串数据的引用消耗更多的时间和内存。
        不过拷贝数据使得代码显得更加直白因为无需管理引用的生命周期，所以在这种情况下牺牲一小部分性能来换取简洁性的取舍是值得的。

        由于其运行时消耗，许多 Rustacean 之间有一个趋势是倾向于避免使用 clone 来解决所有权问题。
        在关于迭代器的第十三章中，我们将会学习如何更有效率的处理这种情况，
        不过现在，复制一些字符串来取得进展是没有问题的，因为只会进行一次这样的拷贝，而且文件名和要搜索的字符串都比较短。

        在第一轮编写时拥有一个可以工作但有点低效的程序要比尝试过度优化代码更好一些。
        随着你对 Rust 更加熟练，将能更轻松的直奔合适的方法，不过现在调用 clone 是完全可以接受的。
        */

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