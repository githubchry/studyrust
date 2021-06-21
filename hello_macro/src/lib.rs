
pub trait HelloMacro {
    fn hello_macro();
}
/*
现在有了一个包含函数的 trait 。此时，crate 用户可以实现该 trait 以达到其期望的功能，像这样：
    use hello_macro::HelloMacro;

    struct Pancakes;

    impl HelloMacro for Pancakes {
        fn hello_macro() {
            println!("Hello, Macro! My name is Pancakes!");
        }
    }

    fn main() {
        Pancakes::hello_macro();
    }

然而，他们需要为每一个他们想使用 hello_macro 的类型编写实现的代码块。我们希望为其节约这些工作。
另外，我们也无法为 hello_macro 函数提供一个能够打印实现了该 trait 的类型的名字的默认实现：Rust 没有反射的能力，因此其无法在运行时获取类型名。
我们需要一个在编译时生成代码的宏。

下一步是定义过程式宏。在编写本部分时，过程式宏必须在其自己的 crate 内。(该限制最终可能被取消。)
构造 crate 和其中宏的惯例如下：对于一个 foo 的包来说，一个自定义的派生过程宏的包被称为 foo_derive 。
在 hello_macro 项目中新建名为 hello_macro_derive 的包。
    cargo new hello_macro_derive --lib

由于两个 crate 紧密相关，因此在 hello_macro 包的目录下创建过程式宏的 crate。
如果改变在 hello_macro 中定义的 trait ，同时也必须改变在 hello_macro_derive 中实现的过程式宏。
这两个包需要分别发布，编程人员如果使用这些包，则需要同时添加这两个依赖并将其引入作用域。
我们也可以只用 hello_macro 包而将 hello_macro_derive 作为一个依赖，并重新导出过程式宏的代码。
但现在我们组织项目的方式使编程人员在无需 derive 功能时也能够单独使用 hello_macro。
*/