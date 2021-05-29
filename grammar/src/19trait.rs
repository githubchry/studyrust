/*

trait 类似于其他语言中的常被称为 接口（interfaces）的功能，虽然有一些不同。

trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。
可以通过 trait 以一种抽象的方式定义共享的行为。
可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。
trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

例如，这里有多个存放了不同类型和属性文本的结构体：
    结构体 NewsArticle 用于存放发生于世界各地的新闻故事，
    结构体 Tweet（推特） 最多只能存放 280 个字符的内容，以及像是否转推或是否是对推友的回复这样的元数据。
*/
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug)]    //增加注解来派生 Debug trait 以打印枚举内结构体数据
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
/*
我们想要创建一个多媒体聚合库用来显示可能储存在 NewsArticle 或 Tweet 实例中的数据的总结。
每一个结构体都需要的行为是他们是能够被总结的，这样的话就可以调用实例的 summarize 方法来请求总结。

下面展示了一个表现这个概念的 Summary trait 的定义：
*/
// 使用 trait 关键字来声明一个 trait，后面是 trait 的名字 Summary
pub trait Summary {
    // 大括号中声明描述实现这个 trait 的类型所需要的行为的方法签名
    // trait 体中可以有多个方法：一行一个方法签名且都以分号结尾。
    fn summarize(&self) -> String;
    // fn summarize_author(&self) -> String;

    // 有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。
    // 这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。
    fn summarize_source(&self) -> String {
        String::from("群里转的...")
    }
}

// ========================================================================
// 为类型实现 trait
// ========================================================================
/*
前面定义了 Summary trait，接着就可以在多媒体聚合库中需要拥有这个行为的类型上实现它了。
在类型上实现 trait 类似于实现与 trait 无关的方法。
区别在于 impl 关键字之后，我们提供需要实现 trait 的名称，接着是 for 和需要实现 trait 的类型的名称。
在 impl 块中，使用 trait 定义中的方法签名，不过不再后跟分号，而是需要在大括号中编写函数体来为特定类型实现 trait 方法所拥有的行为。
*/
impl Summary for NewsArticle {
    // 使用标题、作者和创建的位置作为 summarize 的返回值
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    // 对默认实现进行重载 注意无法从相同方法的重载实现中调用默认方法。
    fn summarize_source(&self) -> String {
        String::from("不是转载的！！！")
    }
}

impl Summary for Tweet {
    // 用户名后跟推文的全部文本作为返回值，并假设推文内容已经被限制为 280 字符以内。
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
/*
注意因为我们在同一个bin或lib里定义了 Summary trait 和 NewsArticle 与 Tweet 类型，所以他们是位于同一作用域的。

如果这个 bin或lib 是对应 aggregator crate 的，而别人想要利用我们 crate 的功能为其自己的库作用域中的结构体实现 Summary trait。
首先他们需要将 trait 引入作用域。
这可以通过指定 use aggregator::Summary; 实现，这样就可以为其类型实现 Summary trait 了。
Summary 还必须是公有 trait 使得其他 crate 可以实现它，这也是为什么实例中将 pub 置于 trait 之前。

实现 trait 时需要注意的一个限制是，只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。
例如，可以为 aggregator crate 的自定义类型 Tweet 实现如标准库中的 Display trait，这是因为 Tweet 类型位于 aggregator crate 本地的作用域中。
类似地，也可以在 aggregator crate 中为 Vec<T> 实现 Summary，这是因为 Summary trait 位于 aggregator crate 本地作用域中。

但是不能为外部类型实现外部 trait。
例如，不能在 aggregator crate 中为 Vec<T> 实现 Display trait。
这是因为 Display 和 Vec<T> 都定义于标准库中，它们并不位于 aggregator crate 本地作用域中。
这个限制是被称为 相干性（coherence） 的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型。
这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。
没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。
*/

// ========================================================================
// trait 作为参数
// ========================================================================
/*
知道了如何定义 trait 和在类型上实现这些 trait 之后，我们可以探索一下如何使用 trait 来接受多种不同类型的参数。
定义一个函数 notify 来调用其参数 item 上的 summarize 方法，该参数是实现了 Summary trait 的某种类型。
为此可以使用 impl Trait 语法，像这样：
*/
pub fn notify(item: impl Summary) {
    // item 参数支持任何实现了指定 trait 的类型。
    // 在函数体中，可以使用item调用任何来自 Summary trait 的方法，比如 summarize。
    println!("Breaking news! {}", item.summarize());
}

// ========================================================================
// Trait Bound 语法
// ========================================================================
/*
impl Trait 语法适用于直观的例子，是一个`较长形式`的语法糖
这个`较长形式`被称为 trait bound, 与泛型参数声明在一起，位于尖括号中的冒号后面。
长这样：
*/
pub fn trait_bound_notify<T: Summary>(item: T) {
    // 比impl Summary形式稍微冗长了一些。调用形式亦与notify一致
    println!("Breaking news! {}", item.summarize());
}
/*
impl Trait 很方便，适用于短小的例子。trait bound 则适用于更复杂的场景。
例如，可以获取两个实现了 Summary 的参数。使用 impl Trait 的语法看起来像这样：

    pub fn notify(item1: impl Summary, item2: impl Summary) {

这适用于 item1 和 item2 允许是不同类型的情况（只要它们都实现了 Summary）。
不过如果你希望强制它们都是相同类型呢？这只有在使用 trait bound 时才有可能：

    pub fn notify<T: Summary>(item1: T, item2: T) {

泛型 T 被指定为 item1 和 item2 的参数限制，如此传递给参数 item1 和 item2 值的具体类型必须一致。
*/

// ========================================================================
// 通过加号`+`指定多个 trait bound
// ========================================================================
// 如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，
// 那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现：
use std::fmt::Display;

// + 语法也适用于泛型的 trait bound：
// pub fn notify_ex<T: Summary + Display>(item: T) {
pub fn notify_ex(item: impl Summary + Display) {
    // 比impl Summary形式稍微冗长了一些。调用形式亦与notify一致
    println!("Breaking news! {}", item.summarize());
    // println!("notify_ex item {:?}", item);
}
// 这里仅仅示例语法 实际应用需要实现或引入确切用到的trait，详见18generices.rs的largest函数


// ========================================================================
// 通过 where 简化 trait bound
// ========================================================================
/*
每个泛型有其自己的 trait bound，所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，这使得函数签名难以阅读。
为此，Rust 有另一个在函数签名之后的 where 从句中指定 trait bound 的语法。所以除了这么写：

    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

还可以像这样使用 where 从句：

    fn some_function<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    {

这个函数签名就显得不那么杂乱，函数名、参数列表和返回值类型都离得很近，看起来类似没有很多 trait bounds 的函数。
 */
// ========================================================================
// 返回实现了 trait 的类型
// ========================================================================
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("chry"),
        content: String::from("5 月 29 日消息，近日特斯拉宣布在北美将主动召回 2018 年 12 月至..."),
        reply: false,
        retweet: false,
    }
}
/*
返回一个只是指定了需要实现的 trait 的类型的能力在闭包和迭代器场景十分的有用，第十三章会介绍它们。
闭包和迭代器创建只有编译器知道的类型，或者是非常非常长的类型。
impl Trait 允许你简单的指定函数返回一个 Iterator 而无需写出实际的冗长的类型。

不过这只适用于返回单一类型的情况。
例如，这段代码的返回值类型指定为返回 impl Summary，但是返回了 NewsArticle 或 Tweet 就行不通：

fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
这里尝试返回 NewsArticle 或 Tweet。这不能编译，因为 impl Trait 工作方式的限制。
第十七章的 “为使用不同类型的值而设计的 trait 对象” 部分会介绍如何编写这样一个函数。
*/

// ========================================================================
// 使用 trait bound 有条件地实现方法
// ========================================================================
/*
通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法。
例如，示例中的类型 Pair<T> 总是实现了 new 方法，
不过只有那些为 T 类型实现了 PartialOrd trait （来允许比较） 和 Display trait （来启用打印）的 Pair<T> 才会实现 cmp_display 方法：
*/
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
/*
也可以对任何实现了特定 trait 的类型有条件地实现 trait。
对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations，他们被广泛的用于 Rust 标准库中。
例如，标准库为任何实现了 Display trait 的类型实现了 ToString trait。这个 impl 块看起来像这样：

impl<T: Display> ToString for T {
    // --snip--
}

因为标准库有了这些 blanket implementation，我们可以对任何实现了 Display trait 的类型调用由 ToString 定义的 to_string 方法。
例如，可以将整型转换为对应的 String 值，因为整型实现了 Display：

let s = 3.to_string();

blanket implementation 会出现在 trait 文档的 “Implementers” 部分。

trait 和 trait bound 让我们使用泛型类型参数来减少重复，并仍然能够向编译器明确指定泛型类型需要拥有哪些行为。
因为我们向编译器提供了 trait bound 信息，它就可以检查代码中所用到的具体类型是否提供了正确的行为。
在动态类型语言中，如果我们尝试调用一个类型并没有实现的方法，会在运行时出现错误。
Rust 将这些错误移动到了编译时，甚至在代码能够运行之前就强迫我们修复错误。
另外，我们也无需编写运行时检查行为的代码，因为在编译时就已经检查过了，这样相比其他那些不愿放弃泛型灵活性的语言有更好的性能。
*/

fn main() {
    let tweet = returns_summarizable();

    let it_home = NewsArticle {
        headline: String::from("特斯拉：召回不涉及国产车型，也与“刹车失灵”无关"),
        location: String::from("汽车之家"),
        content: String::from("5 月 29 日消息，近日特斯拉宣布在北美将主动召回 2018 年 12 月至..."),
        author: String::from("网易科技(-)"),
    };

    // 一旦实现了 trait，我们就可以用与 NewsArticle 和 Tweet 实例的非 trait 方法一样的方式调用 trait 方法了：
    println!("1 new tweet: {}", tweet.summarize_source());
    notify(tweet);

    println!("1 new it home news: {}", it_home.summarize_source());
    trait_bound_notify(it_home);
}