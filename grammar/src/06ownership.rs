/*
所有权系统：跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间

所有权规则：
    1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    2. 值在任一时刻有且只有一个所有者。
    3. 当所有者（变量）离开作用域，这个值将被丢弃。
*/

fn main() {
    // ========================================================================
    // 变量作用域: 声明之始，大括号内
    // ========================================================================
    // 字符串字面值
    {
        // s 在这里无效, 它尚未声明
        let s = "hello"; //从此处起，s 是有效的
        println!("{}", s);
    } //离开作用域，s 不再有效

    // println!("{}", s);   //编译错误

    // 字符串指针值
    {
        // String类型被分配到堆上，所以能够存储在编译时未知大小的文本。
        let mut s = String::from("hello");
        // 两个冒号（::）是运算符，允许将特定的 from 函数置于 String 类型的命名空间（namespace）下，而不需要使用类似 string_from 这样的名字
        s.push_str(", world!"); // push_str() 在字符串后追加字面值
        println!("{}", s); // 将打印 `hello, world!`
    } //离开作用域，生命周期结束，RAII机制自动调用drop函数（C++的析构）

    // ========================================================================
    // 变量与数据交互的方式（一）：移动 move
    // ========================================================================
    println!("\n变量与数据交互的方式（一）：移动 move");
    let s1 = String::from("hello");
    println!("{} {:p} {:p}", s1, &s1, s1.as_ptr());
    let s2 = s1;
    println!("{} {:p} {:p}", s2, &s2, s2.as_ptr());

    // println!("{} ", s1);     // 编译错误：s1上数据的所有权被移动到s2上 s1不再有效

    // ========================================================================
    // 变量与数据交互的方式（二）：克隆 clone
    // ========================================================================
    println!("\n变量与数据交互的方式（二）：克隆 clone");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} {:p} {:p}", s1, &s1, s1.as_ptr());
    println!("{} {:p} {:p}", s2, &s2, s2.as_ptr());

    // ========================================================================
    // 只在栈上的数据：拷贝 copy
    // ========================================================================
    println!("\n只在栈上的数据：拷贝 copy");
    let x = 5;
    let y = x;
    println!("{} {} {:p} {:p}", x, y, &x, &y);
    /*
    像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。
    这意味着没有理由在创建变量 y 后使 x 无效。

    任何简单标量值的组合可以是 Copy 的，不需要分配内存或某种形式资源的类型是 Copy 的:
        整数, 布尔，浮点，字符char
        元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
    */

    // ========================================================================
    // 所有权与函数
    // ========================================================================
    println!("\n所有权与函数");
    let s = String::from("hello"); // s 进入作用域
    println!("{} {:p} {:p}", s, &s, s.as_ptr());
    takes_ownership(s); // s 的值移动到函数里 ...

    // println!("{} ", s);     // 编译错误：s上数据的所有权被移动到函数里面 s不再有效

    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，但 i32 是 Copy 的，所以在后面可继续使用 x
    println!("{} ", x);

    // ========================================================================
    // 返回值与作用域
    // ========================================================================
    println!("\n返回值与作用域");
    // 返回值也可以转移所有权
    let s1 = gives_ownership(); // gives_ownership 将返回值移给 s1
    println!("{} {:p} {:p} => s1 一毛一样\n", s1, &s1, s1.as_ptr());

    let s2 = String::from("hello"); // s2 进入作用域
    println!("{} {:p} {:p} => s2", s2, &s2, s2.as_ptr());

    let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中, 它也将返回值移给 s3
    println!("{} {:p} {:p} => s3", s3, &s3, s3.as_ptr());
    println!("所有者变来变去, 但String下的指针不变\n");
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!(
        "{} {:p} {:p}",
        some_string,
        &some_string,
        some_string.as_ptr()
    );
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

// gives_ownership 将返回值移动给调用它的函数
fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string 进入作用域.
    println!(
        "{} {:p} {:p} => gives_ownership some_string",
        some_string,
        &some_string,
        some_string.as_ptr()
    );
    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    println!(
        "{} {:p} {:p} => takes_and_gives_back a_string",
        a_string,
        &a_string,
        a_string.as_ptr()
    );
    a_string // 返回 a_string 并移出给调用的函数
}
