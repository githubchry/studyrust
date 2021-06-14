/*
创建一个图形用户接口（Graphical User Interface， GUI）工具的例子，
它通过遍历列表并调用每一个项目的 draw 方法来将其绘制到屏幕上 —— 此乃一个 GUI 工具的常见技术。
我们将要创建一个叫做 gui 的库 crate，它含一个 GUI 库的结构。这个 GUI 库包含一些可供开发者使用的类型，比如 Button 或 TextField。
在此之上，gui 的用户希望创建自定义的可以绘制于屏幕上的类型：比如，一个程序员可能会增加 Image，另一个可能会增加 SelectBox。

这个例子中并不会实现一个功能完善的 GUI 库，不过会展示其中各个部分是如何结合在一起的。
编写库的时候，我们不可能知晓并定义所有其他程序员希望创建的类型。
我们所知晓的是 gui 需要记录一系列不同类型的值，并需要能够对其中每一个值调用 draw 方法。
这里无需知道调用 draw 方法时具体会发生什么，只要该值会有那个方法可供我们调用。

在拥有继承的语言中，可以定义一个名为 Component 的类，该类上有一个 draw 方法。
其他的类比如 Button、Image 和 SelectBox 会从 Component 派生并因此继承 draw 方法。
它们各自都可以覆盖 draw 方法来定义自己的行为，但是框架会把所有这些类型当作是 Component 的实例，并在其上调用 draw。
不过 Rust 并没有继承，我们得另寻出路。
 */

// 为了实现 gui 所期望的行为，让我们定义一个 Draw trait，其中包含名为 draw 的方法。
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // 定义一个存放 Draw trait 对象的 vector。
    pub components: Vec<Box<dyn Draw>>,
}

// 在 Screen 结构体上，我们将定义一个 run 方法，该方法会对其 components 上的每一个组件调用 draw 方法
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 自带 Button 类型
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

//  实现 trait
impl Draw for Button {
    fn draw(&self) {
        // 实际绘制按钮的代码
        println!("在屏幕画了一个宽{} 高{}的{}按钮", self.width, self.height, self.label);
    }
}

// 其他控件类型在main.rs由用户自定义开发....



