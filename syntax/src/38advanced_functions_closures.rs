
// ========================================================================
// 函数指针
// ========================================================================
// 通过函数指针允许我们使用函数作为另一个函数的参数。函数的类型是 fn （使用小写的 “f” ）以免与 Fn 闭包 trait 相混淆。
// fn 被称为 函数指针（function pointer）。指定参数为函数指针的语法类似于闭包，

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    // 签名中的 f 被指定为一个接受一个 i32 参数并返回 i32 的 fn。接着就可以在函数体中调用 f
    f(arg) + f(arg)
}

// cargo test --bin 38advanced_functions_closures
#[test]
fn test_outline_print_point() {

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    let value = do_twice(add_one, 5);

    assert_eq!(12, value);
}

/*
不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数。

函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数。
倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函数或闭包作为参数。

一个只期望接受 fn 而不接受闭包的情况的例子是与不存在闭包的外部代码交互时：C 语言的函数可以接受函数作为参数，但 C 语言没有闭包。

作为一个既可以使用内联定义的闭包又可以使用命名函数的例子，让我们看看一个 map 的应用。
使用 map 函数将一个数字 vector 转换为一个字符串 vector，就可以使用闭包，比如这样：
*/
fn main(){
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    println!("{:?}", list_of_numbers);
    println!("{:?}", list_of_strings);

    // 或者可以将函数作为 map 的参数来代替闭包，像是这样：
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
    // 注意这里必须使用 “高级 trait” 部分讲到的完全限定语法，因为存在多个叫做 to_string 的函数；
    // 这里使用了定义于 ToString trait 的 to_string 函数，标准库为所有实现了 Display 的类型实现了这个 trait。

    println!("{:?}", list_of_strings);

    // 另一个实用的模式 暴露了 元组结构体和元组结构体枚举成员 的实现细节。
    // 这些项使用 () 作为初始化语法，这看起来就像函数调用，同时它们确实被实现为返回由参数构造的实例的函数。
    // 它们也被称为实现了闭包 trait 的函数指针，并可以采用类似如下的方式调用：
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> =
        (0u32..20)
            .map(Status::Value)     // // 通过 map 对0~20范围的每一个 u32 值调用 Status::Value 的初始化函数。
            .collect();

    println!("{:?}", list_of_statuses);
    // 一些人倾向于函数风格，一些人喜欢闭包。这两种形式最终都会产生同样的代码，所以请使用对你来说更明白的形式吧。


    // ========================================================================
    // 返回闭包
    // ========================================================================
    /*
    闭包表现为 trait，这意味着不能直接返回闭包。
    对于大部分需要返回 trait 的情况，可以使用实现了期望返回的 trait 的具体类型来替代函数的返回值。
    但是这不能用于闭包，因为他们没有一个可返回的具体类型；例如不允许使用函数指针 fn 作为返回值类型。
    这段代码尝试直接返回闭包，它并不能编译：
        fn returns_closure() -> Fn(i32) -> i32 {
            |x| x + 1
        }
    编译器给出的错误是：
    the trait bound `std::ops::Fn(i32) -> i32 + 'static:
        std::marker::Sized` is not satisfied

    错误又一次指向了 Sized trait！Rust 并不知道需要多少空间来储存闭包。
    （回顾37advanced_types.rs中的`动态大小类型和 Sized trait`）
    不过我们在上一部分见过这种情况的解决办法：可以使用 trait 对象：
    */
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    let f = returns_closure();
    println!("{}", f(5));
}