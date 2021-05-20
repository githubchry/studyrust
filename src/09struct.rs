// 和元组一样，结构体的每一部分可以是不同类型。但不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。
// 由于有了这些名字，结构体比元组更灵活：不需要依赖顺序来指定或访问实例中的值。
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 变量与字段同名时的字段初始化简写语法
fn build_user(email: String, username: String) -> User {
    User {
        // email: email,        // 结构体字段名与函数入参名相同，可简写
        email,
        // username: username,  // 结构体字段名与函数入参名相同，可简写
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {

    // 结构体在创建实例时必须指定所有成员值
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Rust 并不允许只将结构体某个字段标记为可变
    let mut user1 = user1;
    user1.email = String::from("anotheremail@example.com");

    // 在函数中创建并返回结构体（变量与字段同名时的字段初始化简写语法）
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));

    // 使用结构体更新语法从其他实例创建实例
    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..user1 // `..` 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
    };

    // ========================================================================
    // 元组结构体（tuple structs）
    // ========================================================================
    // 元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    // ========================================================================
    // 类单元结构体（unit-like structs）
    // ========================================================================
    // 一个没有任何字段的结构体被称为 类单元结构体（unit-like structs）因为它们类似于 ()，即 unit 类型。
    // 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
    struct Unit();
    let u = Unit();


    // ========================================================================
    // 结构体数据的所有权
    // ========================================================================
    // User结构体使用了自身拥有所有权的 String 类型而不是 &str 字符串 slice 类型。
    // 这是一个有意而为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。
    struct UserTest {
        // username: &str, // 编译错误：expected lifetime parameter
        email: String,
    }
    // 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes）
    // 生命周期确保结构体引用的数据有效性跟结构体本身保持一致。
    // 关于声明周期 后面再学。


    // ========================================================================
    // 示例：计算长方形的面积
    // ========================================================================
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

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
    struct RectangleTest {
        width: u32,
        height: u32,
    }
    let rect1 = RectangleTest { width: 30, height: 50 };
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
    impl RectangleTest {
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
        fn square(size: u32) -> RectangleTest {
            RectangleTest { width: size, height: size }
        }
    }

    let rect1 = RectangleTest::square(30);

    println!(
        "The area {:?} of the rectangle is {} square pixels.",
        rect1,
        rect1.area()
    );

    /*
    多个 impl 块 : 每个结构体都允许拥有多个 impl 块。
    */
    impl RectangleTest {
        // 如果 self 能完全包含第二个长方形则返回 true；否则返回 false。
        fn can_hold(&self, other: &RectangleTest) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect2 = RectangleTest::square(50);
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

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}