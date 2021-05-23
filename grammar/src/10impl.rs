
fn main() {
    // ========================================================================
    // 通过派生 trait 增加实用功能
    // ========================================================================
    // println!("rect1 is {:?}", rect1);   // 编译错误：无法直接打印结构体 `Rectangle` doesn't implement `std::fmt::Display`
    /*
    println! 宏能处理很多类型的格式，不过，{} 默认告诉 println! 使用被称为 Display 的格式：意在提供给直接终端用户查看的输出。
    目前为止见过的基本类型都默认实现了 Display，因为它就是向用户展示 1 或其他任何基本类型的唯一方式。
    不过对于结构体，println! 应该用来输出的格式是不明确的，因为这有更多显示的可能性：是否需要逗号？需要打印出大括号吗？所有字段都应该显示吗？
    由于这种不确定性，Rust 不会尝试猜测我们的意图，所以结构体并没有提供一个 Display 实现。

    但是如果我们继续阅读错误，将会发现这个有帮助的信息：
        = help: the trait `Debug` is not implemented for `Rectangle`
        = note: add `#[derive(Debug)]` or manually implement `Debug`

    Rust 确实 包含了打印出调试信息的功能，不过我们必须为结构体显式选择这个功能。
    为此，在结构体定义之前加上 #[derive(Debug)] 注解
    */
    #[derive(Debug)]    //增加注解来派生 Debug trait
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);   // 打印：rect1 is RectangleTest { width: 30, height: 50 }
    println!("rect1 is {:#?}", rect1);  // 格式化打印（自动缩进换行）

    // ========================================================================
    // 方法
    // ========================================================================
    // 方法 与函数不同之处：
    //      方法在结构体或者是枚举或 trait 对象的上下文中被定义，并且它们第一个参数总是 self，代表调用该方法的结构体实例。

    // 前面写的 area 函数是非常特殊的，它只计算长方形的面积。
    // 如果这个行为与 Rectangle 结构体再结合得更紧密一些就更好了，因为它不能用于其他类型。
    // 可将 area 函数协调进 Rectangle 类型定义的 area 方法 中
    impl Rectangle {
        /*
        为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块（impl 是 implementation 的缩写）。
        因为该方法位于 impl Rectangle 上下文中所以 Rust 知道 self 的类型是 Rectangle。
        方法的参数，就跟其他参数一样，三种形式：
            获取 self 的所有权 - `area(self)`
                少见，这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。
            不可变地借用 self - `area(&self)`
                并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。
            可变地借用 self - `area(&mut self)`
                如果想要在方法中改变调用方法的实例(写入)，需要将第一个参数改为 &mut self。
                比如RectangleTest还有一个成员字段叫`面积`，想在方法内直接更新`面积`字段成员

        如上，我们选择了 &self ：
        */
        fn area(&self) -> u32 {
            self.width * self.height
        }


        // ========================================================================
        // 关联函数（associated functions）
        // ========================================================================
        /*
        impl 块的另一个有用的功能是：允许在 impl 块中定义 不 以 self 作为参数的函数。
        这被称为 关联函数，因为它们与结构体相关联。它们仍是函数而不是方法，因为它们并不作用于一个结构体的实例。
        你已经使用过 String::from 关联函数了。
        */
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }

    let rect1 = Rectangle::square(30);

    println!(
        "The area {:?} of the rectangle is {} square pixels.",
        rect1,
        rect1.area()
    );

    /*
    多个 impl 块 : 每个结构体都允许拥有多个 impl 块。
    */
    impl Rectangle {
        // 如果 self 能完全包含第二个长方形则返回 true；否则返回 false。
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect2 = Rectangle::square(50);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    // ========================================================================
    // 自动引用和解引用（automatic referencing and dereferencing）
    // ========================================================================
    /*
    在 C/C++ 语言中，有两个不同的运算符来调用方法：. 直接在对象上调用方法，而 -> 在一个对象的指针上调用方法，这时需要先解引用（dereference）指针。
    换句话说，如果 object 是一个指针，那么 object->something() 就像 (*object).something() 一样。

    Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
    方法调用是 Rust 中少数几个拥有这种行为的地方。

    他是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
    也就是说，这些代码是等价的：
        p1.distance(&p2);
        (&p1).distance(&p2);

    第一行看起来简洁的多。这种自动引用的行为之所以有效，是因为方法有一个明确的接收者———— self 的类型。
    在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）。
    事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好。
    */

}