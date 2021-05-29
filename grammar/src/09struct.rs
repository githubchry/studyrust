// 和元组一样，结构体的每一部分可以是不同类型。但不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。
// 由于有了这些名字，结构体比元组更灵活：不需要依赖顺序来指定或访问实例中的值。
// Rust 类型名的命名规范是骆驼命名法（CamelCase）
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
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}