/*
 过程宏（procedural macros）更像函数（一种过程类型），它接收 Rust 代码作为输入，在这些代码上进行操作，
 然后产生另一些代码作为输出，而非像声明式宏那样匹配对应模式然后以另一部分代码替换当前代码。

 有三种类型的过程宏（自定义派生（derive），类属性和类函数），不过它们的工作方式都类似。

 当创建过程宏时，其定义必须位于一种特殊类型的属于它们自己的 crate 中。这么做出于复杂的技术原因，将来我们希望能够消除这些限制。
 使用这些宏需采用类似示例所示的代码形式，其中 some_attribute 是一个使用特定宏的占位符。
    use proc_macro;

    #[some_attribute]
    pub fn some_name(input: TokenStream) -> TokenStream {
    }
过程宏包含一个函数，这也是其得名的原因：“过程” 是 “函数” 的同义词。那么为何不叫 “函数宏” 呢？好吧，有一个过程宏是 “类函数” 的，叫成函数会产生混乱。
无论如何，定义过程宏的函数接受一个 TokenStream 作为输入并产生一个 TokenStream 作为输出。
这也就是宏的核心：宏所处理的源代码组成了输入 TokenStream，同时宏生成的代码是输出 TokenStream。
最后，函数上有一个属性；这个属性表明过程宏的类型。在同一 crate 中可以有多种的过程宏。

考虑到这些宏是如此类似，我们会从自定义派生宏开始。接着会解释与其他形式宏的微小区别。


// ========================================================================
// 编写自定义 derive 宏
// ========================================================================
让我们创建一个 hello_macro crate，其包含名为 HelloMacro 的 trait 和关联函数 hello_macro。
不同于让 crate 的用户为其每一个类型实现 HelloMacro trait，我们将会提供一个过程式宏以便用户可以使用 #[derive(HelloMacro)] 注解他们的类型来得到 hello_macro 函数的默认实现。
该默认实现会打印 Hello, Macro! My name is TypeName!，其中 TypeName 为定义了 trait 的类型名。
*/

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();

    /*
    接下来，让我们探索一下其他类型的过程宏与自定义派生宏有何区别。

    // ========================================================================
    // 类属性宏
    // ========================================================================
    类属性宏与自定义派生宏相似，不同于为 derive 属性生成代码，它们允许你创建新的属性。
    它们也更为灵活；derive 只能用于结构体和枚举；属性还可以用于其它的项，比如函数。
    作为一个使用类属性宏的例子，可以创建一个名为 route 的属性用于注解 web 应用程序框架（web application framework）的函数：
            #[route(GET, "/")]
            fn index() {
    #[route] 属性将由框架本身定义为一个过程宏。其宏定义的函数签名看起来像这样：
            #[proc_macro_attribute]
            pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    这里有两个 TokenStream 类型的参数；第一个用于属性内容本身，也就是 GET, "/" 部分。
    第二个是属性所标记的项：在本例中，是 fn index() {} 和剩下的函数体。

    除此之外，类属性宏与自定义派生宏工作方式一致：创建 proc-macro crate 类型的 crate 并实现希望生成代码的函数！


    // ========================================================================
    // 类函数宏
    // ========================================================================
    类函数宏定义看起来像函数调用的宏。类似于 macro_rules!，它们比函数更灵活；例如，可以接受未知数量的参数。
    然而 macro_rules! 宏只能使用之前 “使用 macro_rules! 的声明宏用于通用元编程” 介绍的类匹配的语法定义。
    类函数宏获取 TokenStream 参数，其定义使用 Rust 代码操纵 TokenStream，就像另两种过程宏一样。
    一个类函数宏例子是可以像这样被调用的 sql! 宏：
        let sql = sql!(SELECT * FROM posts WHERE id=1);
    这个宏会解析其中的 SQL 语句并检查其是否是句法正确的，这是比 macro_rules! 可以做到的更为复杂的处理。sql! 宏应该被定义为如此：
        #[proc_macro]
        pub fn sql(input: TokenStream) -> TokenStream {
    这类似于自定义派生宏的签名：获取括号中的 token，并返回希望生成的代码。
    */
}