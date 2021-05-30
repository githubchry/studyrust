fn main() {
    // 定义枚举
    enum IpAddrKind {
        V4,
        V6,
    }

    // 枚举值
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_type: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // C/C++的枚举实际上就是一个整型数值，通常作为结构体的一部分或者函数参数
    // Rust 还可以使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。
    // 用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据。
    // 我们直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。
    enum IpAddrMini {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrMini::V4(127, 0, 0, 1);
    let loopback = IpAddrMini::V6(String::from("::1"));

    // 还可以将任意类型的数据放入枚举成员中：例如字符串、数字类型或者结构体。甚至可以包含另一个枚举
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddrCustom {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }


    // 一个 Message 枚举，其每个成员都存储了不同数量和类型的值
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // 结构体和枚举还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法。
    // 这是一个定义于我们 Message 枚举上的叫做 call 的方法：
    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // ========================================================================
    // Option 枚举和其相对于空值的优势
    // ========================================================================
    /*
    在有空值的语言中，变量总是这两种状态之一：空值和非空值。
    C/C++因为空值的存在引发了无数错误、漏洞和系统崩溃......Rust 并没有很多其他语言中有的空值功能。

    所以Rust完全不用担心这样的错误，写起来可以很自信！

    然而，空值尝试表达的概念仍然是有意义的：空值是一个因为某种原因目前无效或缺失的值。
    问题不在于概念而在于具体的实现。为此，Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。
    这个枚举是 Option<T>
    Option 是标准库定义的另一个枚举。Option 类型应用广泛因为它编码了一个非常普遍的场景，即一个值要么有值要么没值。
    enum Option<T> {
        Some(T),
        None,
    }
    */


    // 当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。
    // 当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值。
    let some_number = Some(5);  //自动推导出T是i32
    let some_string = Some("a string"); //自动推导出T是&str
    // 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
    let absent_number: Option<i32> = None;

    // Option<T> 和 T（这里 T 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 Option<T>。
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;    //编译错误：i8与Option<i8>不是同一类型

    /*
    当在 Rust 中拥有一个像 i8 这样类型的值时，编译器确保它总是有一个有效的值。我们可以自信使用而无需做空值检查。
    只有当使用 Option<i8>（或者任何用到的类型）的时候需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况。

    换句话说，在对 Option<T> 进行 T 的运算之前必须将其转换为 T。
    通常这能帮助我们捕获到空值最常见的问题之一：假设某值不为空但实际上为空的情况。

    match 和 if let 语法可以帮助我们对枚举进行解构，详见 12match_if-let.rs
    */

    // 将Option<i8>转为i8
    let mut y_val = 0;  //通过上下文自动推导出类型为i8
    if let Some(i) = y {
        y_val = i;
    }

    let sum = x + y_val;
    println!("{} + {} + {}", x, y_val, sum)
}