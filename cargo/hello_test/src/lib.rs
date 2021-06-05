// cargo test
// cargo test this_test_will_pass
// cargo test this_test_will_fail
// cargo test this_test_will_pass --  --nocapture

// 公有函数可供外部集成测试（tests目录下use引用并测试）
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

// 创建的 Guess 类型。其他使用 Guess 的代码都是基于 Guess 实例仅有的值范围在 1 到 100 的前提。
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 /*|| value > 100*/ {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

// 私有函数只能在单元测试里面测（本文件）
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

/*
测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，而在运行 cargo build 时不这么做。
这在只希望构建库的时候可以节省编译时间，并且因为它们并没有包含测试，所以能减少编译产生的文件的大小。
与之对应的集成测试因为位于另一个文件夹，所以它们并不需要 #[cfg(test)] 注解。
然而单元测试位于与源码相同的文件中，所以你需要使用 #[cfg(test)] 来指定他们不应该被包含进编译结果中。

cfg 属性代表 configuration ，它告诉 Rust 其之后的项只应该被包含进特定配置选项中。
在这个例子中，配置选项是 test，即 Rust 所提供的用于编译和运行测试的配置选项。
通过使用 cfg 属性，Cargo 只会在我们主动使用 cargo test 运行测试时才编译测试代码。
需要编译的不仅仅有标注为 #[test] 的函数之外，还包括测试模块中可能存在的帮助函数。
*/
#[cfg(test)]
mod tests {
    #[test]
    fn another() { panic!("Make this test fail!"); }    // 模拟一个失败的测试

    // ========================================================================
    // 使用 assert! 宏来检查结果
    // ========================================================================
    /*
    assert! 宏帮助我们检查代码是否以期望的方式运行。
    assert! 宏由标准库提供，在希望确保测试中一些条件为 true 时非常有用。需要向 assert! 宏提供一个求值为布尔值的参数。
    如果值是 true，assert! 什么也不做，同时测试会通过。如果值为 false，assert! 调用 panic! 宏，这会导致测试失败。

    Rectangle 的 can_hold 方法返回一个布尔值，这意味着它完美符合 assert! 宏的使用场景

    tests 是一个普通的模块，要测试外部模块中的代码，需要将其引入到内部模块的作用域中。
    这里选择使用 glob 全局导入，以便在 tests 模块中使用所有在外部模块定义的内容。
     */
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    // ========================================================================
    // 使用 assert_eq! 和 assert_ne! 宏来测试相等
    // ========================================================================
    // 这两个宏分别比较两个值是相等还是不相等。
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 5);
    }

    // ========================================================================
    // 自定义失败信息
    // ========================================================================
    // 可以向 assert!、assert_eq! 和 assert_ne! 宏传递一个可选的失败信息参数，可以在测试失败时将自定义失败信息一同打印出来。
    // 三者必需参数之后指定的参数都会传递给 format! 宏，所以可以传递一个包含 {} 占位符的格式字符串和需要放入占位符的值。
    // 自定义信息有助于记录断言的意义；当测试失败时就能更好的理解代码出了什么问题。
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
        assert_eq!(2 + 2, 4, "2 + 2 != 4....{}", "nb");
        assert_ne!(2 + 2, 5, "2 + 2 = 5....{}", "hehe");
    }

    // ========================================================================
    // 使用 should_panic 检查 panic
    // ========================================================================
    // 除了检查代码是否返回期望的正确的值之外，检查代码是否按照期望处理错误也是很重要的。
    // 编写一个测试来确保创建一个超出范围的值的 Guess 实例会 panic。
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // ========================================================================
    // 将 Result<T, E> 用于测试
    // ========================================================================
    // 不同于调用 assert_eq! 宏, 使用 Result<T, E> 作为返回，测试通过时返回 Ok(())，在测试失败时返回带有 String 的 Err。
    // 这样编写测试来返回 Result<T, E> 就可以在函数体中使用问号运算符，如此可以方便的编写任何运算符会返回 Err 成员的测试。
    // 不能对这些使用 Result<T, E> 的测试使用 #[should_panic] 注解。相反应该在测试失败时直接返回 Err 值。
    #[test]
    fn works() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // ========================================================================
    // 显示函数输出
    // ========================================================================
    /*
    默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容。
    比如在测试中调用了 println! 而测试通过了，我们将不会在终端看到 println! 的输出：只会看到说明测试通过的提示行。
    如果测试失败了，则会看到所有标准输出和其他错误信息。

    如果你希望也能看到通过的测试中打印的值，截获输出的行为可以通过 --nocapture 参数来禁用：

    cargo test this_test_will_pass
    cargo test this_test_will_fail
    cargo test this_test_will_pass --  --nocapture
    */
    #[test]
    fn this_test_will_pass() {
        // 测试社区中一直存在关于是否应该对私有函数直接进行测试的论战，而在其他语言中想要测试私有函数是一件困难的，甚至是不可能的事。
        // 不过无论你坚持哪种测试意识形态，Rust 的私有性规则确实允许你测试私有函数
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // ========================================================================
    // 忽略某些测试
    // ========================================================================
    // 有时一些特定的测试执行起来是非常耗费时间的，所以在大多数运行 cargo test 的时候希望能排除他们。
    // 虽然可以通过参数列举出所有希望运行的测试来做到，也可以使用 ignore 属性来标记耗时的测试并排除他们
    // expensive_test 被列为 ignored，如果我们只希望运行被忽略的测试，可以使用 cargo test -- --ignored：
    // cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // 需要运行一个小时的代码
    }
}

