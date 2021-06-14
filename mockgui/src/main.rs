use mockgui::Draw;

// 根据lib给出的 Draw trait，可实现自定义类型

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// 为SelectBox实现Draw trait
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("在屏幕画了一个宽{} 高{}的选择框：{:?}", self.width, self.height, self.options);
    }
}

// 至此 一个自定义控件类型就做好了
// 这个概念 —— 只关心值所反映的信息而不是其具体类型 —— 类似于动态类型语言中称为 鸭子类型（duck typing）的概念：
// 如果它走起来像一只鸭子，叫起来像一只鸭子，那么它就是一只鸭子！

use mockgui::{Screen, Button};

fn main() {
    // 库使用者现在可以在他们的 main 函数中创建一个 Screen 实例。
    // 至此可以通过将 SelectBox 和 Button 放入 Box<T> 转变为 trait 对象来增加组件。
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // 接着可以调用 Screen 的 run 方法，它会调用每个组件的 draw 方法。
    screen.run();
    /*
    Screen 上的 run 实现中，run 并不需要知道各个组件的具体类型是什么。它并不检查组件是 Button 或者 SelectBox 的实例。
    通过指定 Box<dyn Draw> 作为 components vector 中值的类型，我们就定义了 Screen 为需要可以在其上调用 draw 方法的值。

    使用 trait 对象和 Rust 类型系统来进行类似鸭子类型操作的优势是：
    无需在运行时检查一个值是否实现了特定方法或者担心在调用时因为值没有实现方法而产生错误。
    如果值没有实现 trait 对象所需的 trait 则 Rust 不会编译这些代码。
    */
}

/*
// ========================================================================
// Trait 对象执行动态分发
// ========================================================================
对泛型使用 trait bound 时编译器所进行单态化处理：编译器为每一个被泛型类型参数代替的具体类型生成了非泛型的函数和方法实现。
单态化所产生的代码进行 静态分发（static dispatch）。静态分发发生于编译器在编译时就知晓调用了什么方法的时候。
这与 动态分发 （dynamic dispatch）相对，这时编译器在编译时无法知晓调用了什么方法。
在动态分发的情况下，编译器会生成在运行时确定调用了什么方法的代码。

当使用 trait 对象时，Rust 必须使用动态分发。
编译器无法知晓所有可能用于 trait 对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。
为此，Rust 在运行时使用 trait 对象中的指针来知晓需要调用哪个方法。
动态分发也阻止编译器有选择的内联方法代码，这会相应的禁用一些优化。

所以需要对性能与额外的灵活性进行权衡取舍。


// ========================================================================
// Trait 对象要求对象安全
// ========================================================================
只有 对象安全（object safe）的 trait 才可以组成 trait 对象。
围绕所有使得 trait 对象安全的属性存在一些复杂的规则，不过在实践中，只涉及到两条规则。
如果一个 trait 中所有的方法有如下属性时，则该 trait 是对象安全的：
    1. 返回值类型不为 Self
    2. 方法没有任何泛型类型参数
Self 关键字是我们要实现 trait 或方法的类型的别名。
对象安全对于 trait 对象是必须的，因为一旦有了 trait 对象，就不再知晓实现该 trait 的具体类型是什么了。
如果 trait 方法返回具体的 Self 类型，但是 trait 对象忘记了其真正的类型，那么方法不可能使用已经忘却的原始具体类型。
同理对于泛型类型参数来说，当使用 trait 时其会放入具体的类型参数：此具体类型变成了实现该 trait 的类型的一部分。
当使用 trait 对象时其具体类型被抹去了，故无从得知放入泛型参数类型的类型是什么。

一个 trait 的方法不是对象安全的例子是标准库中的 Clone trait。Clone trait 的 clone 方法的参数签名看起来像这样：

pub trait Clone {
    fn clone(&self) -> Self;
}
clone 的签名需要知道什么类型会代替 Self，因为这是它的返回值。
String 实现了 Clone trait，当在 String 实例上调用 clone 方法时会得到一个 String 实例。

 */