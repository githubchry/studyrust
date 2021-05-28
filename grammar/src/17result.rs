/*
Rust 标准库中有很多叫做 Result 的类型：一个通用的 Result 以及在子模块中的特化版本，比如 io::Result。
Result 类型是 枚举（enumerations）, 成员是 Ok 和 Err

enum Result<T, E> {
    Ok(T),  // 表示操作成功，内部包含成功时产生的值。T 代表成功时返回的 Ok 成员中的数据的类型
    Err(E), // 表示操作失败，并且包含失败的前因后果。E 代表失败时返回的 Err 成员中的错误的类型。
}

*/

fn main() {
    // 打开一个不存在的文件
    use std::fs::File;
    let f = File::open("hello.txt");
    /*
    在调用任何一个函数之前，首先必须指导这个函数的返回，两种途径：
        1. 标准库 API 文档： https://doc.rust-lang.org/std/index.html
        2. 直接问编译器：故意指定异常参数，然后编译看错误打印
          --> src/17result.rs:15:16
           |
        15 |     let f:i8 = File::open("hello.txt");
           |           --   ^^^^^^^^^^^^^^^^^^^^^^^ expected `i8`, found enum `Result`
           |           |
           |           expected due to this
           |
           = note: expected type `i8`
                      found enum `Result<File, std::io::Error>`

    得知 File::open 函数的返回值类型是 Result<File, std::io::Error>
    当 File::open 成功的情况下，变量 f 的值将会是一个包含文件句柄的 Ok 实例。
    在失败的情况下，f 的值会是一个包含更多关于出现了何种错误信息的 Err 实例。

    根据 File::open 返回值进行不同处理的逻辑:
    let f = match f {
        // 注意与 Option 枚举一样，Result 枚举和其成员也被导入到了 prelude 中，所以就不需要在 match 分支中的 Ok 和 Err 之前指定 Result::。
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };

    更佳的方案是，对不同的错误原因采取不同的行为：如果 File::open 因为文件不存在而失败，我们希望创建这个文件并返回新文件的句柄。
    */
    use std::io::ErrorKind;
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            //io::ErrorKind 是一个标准库提供的枚举，它的成员对应 io 操作可能导致的不同错误类型。
            // 我们感兴趣的成员是 ErrorKind::NotFound，它代表尝试打开的文件并不存在。
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    println!("{:?}", f);

    // Result<T, E> 有很多接受闭包（closure）的方法，并采用 match 表达式实现。更老练的写法：
    // 一样的行为，但并没有包含任何 match 表达式且更容易阅读。
    // 标准库 API 文档查看unwrap_or_else的作用：Returns the contained Ok value or computes it from a closure.
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f);

    // ========================================================================
    // 失败时 panic 的简写：unwrap 和 expect
    // ========================================================================
    /*
    match 能够胜任它的工作，不过它可能有点冗长并且不总是能很好的表明其意图。
    Result<T, E> 类型定义了很多辅助方法来处理各种情况。比如unwrap 和 expect。
    unwrap：
        如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
        如果 Result 是成员 Err，unwrap 会为我们调用 panic!。
    expect：用来调用 panic! 的错误信息将会作为参数传递给 expect ，而不像unwrap 那样使用默认的 panic! 信息。

    */
    // let f = File::open("null.txt").unwrap();
    // let f = File::open("null.txt").expect("Failed to open null.txt");
    // println!("{:?}", f);


    // ========================================================================
    // 传播（propagating）错误的简写：? 运算符
    // ========================================================================
    /*
    当编写一个其实现会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理。
    这样能更好的控制代码调用，因为比起你代码所拥有的上下文，调用者可能拥有更多信息或逻辑来决定应该如何处理错误。

    一个从文件中读取用户名的函数。如果文件不存在或不能读取，这个函数会将这些错误返回给调用它的代码：
    */
    use std::io;
    use std::io::Read;
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),    // open失败直接返回 Err(e)
        };

        let mut s = String::new();

        // read_to_string 方法也返回一个 Result 因为它也可能会失败
        match f.read_to_string(&mut s) {
            // 如果 read_to_string 成功了，那么这个函数就成功了，并返回文件中的用户名，它现在位于被封装进 Ok 的 s 中。
            // 如果read_to_string 失败了，则像之前处理 File::open 的返回值的 match 那样返回错误值。
            // 不过并不需要显式的调用 return，因为这是函数的最后一个表达式。
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // 这种传播错误的模式在 Rust 是如此的常见，以至于 Rust 提供了 ? 问号运算符来使其更易于处理。
    fn read_username_from_file_shorthand() -> Result<String, io::Error> {
        // File::open 调用结尾的 ? 将会把 Ok 中的值返回给变量 f。
        // 如果出现了错误，? 运算符会提早返回整个函数并将一些 Err 值传播给调用者。
        // 同理也适用于 read_to_string 调用结尾的 ?。
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // 更符合工程学(ergonomic)的写法。在 ? 之后直接使用链式方法调用来进一步缩短代码
    fn read_username_from_file_shorthand_ex() -> Result<String, io::Error> {
        let mut s = String::new();

        // 对 File::open("hello.txt")? 的结果直接链式调用了 read_to_string，而不再创建中间变量 f
        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    /*
    釜底抽薪 终极版本
    将文件读取到一个字符串是相当常见的操作，所以 Rust 提供了名为 fs::read_to_string 的函数，
    它会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它。
    当然，这样做就没有展示所有这些错误处理的机会了，所以我们最初就选择了艰苦的道路。
    */
    fn read_username_from_file_ultimate() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
    /*
    可以回头看看这四个函数的迭代过程，感受一下 Rust 的独特之处
    read_username_from_file
    read_username_from_file_shorthand
    read_username_from_file_shorthand_ex
    read_username_from_file_ultimate
    */

    // ========================================================================
    // ? 运算符只能被用于返回 Result 的函数
    // ========================================================================

    // let f = File::open("hello.txt")?; // 编译错误：main函数的返回值不是Result

    /*
    main 函数是特殊的，其必须返回什么类型是有限制的。main 函数的一个有效的返回值是 ()，同时出于方便，另一个有效的返回值是 Result<T, E>，如下所示：

    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }

    Box<dyn Error> 被称为 “trait 对象”（“trait object”），第十七章 “为使用不同类型的值而设计的 trait 对象” 部分会做介绍。
    目前可以理解 Box<dyn Error> 为使用 ? 时 main 允许返回的 “任何类型的错误”。
    */
}