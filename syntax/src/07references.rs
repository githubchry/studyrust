    /*
引用的规则
    在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    引用必须总是有效的。
*/

fn main() {
    // ========================================================================
    // 引用
    // ========================================================================
    let s = String::from("hello");
    println!("{:p} {:p} {:02} => s : {} ", &s, s.as_ptr(), s.len(), s);

    // & 符号就是 引用，它们允许你使用值但不获取其所有权
    // 与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符 *
    calculate_references_length(&s);
    println!("{:p} {:p} {:02} => s : {} ", &s, s.as_ptr(), s.len(), s);

    let mut s = s;
    println!("修改 String 长度观察其指向的内存地址：");
    println!("{:p} {:p} {:02} => s : {} ", &s, s.as_ptr(), s.len(), s);
    s.push_str("6789012345678901234"); //此时长度为24
    println!("{:p} {:p} {:02} => s : {} ", &s, s.as_ptr(), s.len(), s);
    s.push_str("1"); // 此时长度为25，原有内存放不下，申请新的更大内存，并把旧数据拷贝过去，添加新数据
    println!("{:p} {:p} {:02} => s : {} ", &s, s.as_ptr(), s.len(), s);

    // 变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。
    // let change = |s: &String| {s.push_str(", world");};  // 编译错误：`s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // change(&s);

    // ========================================================================
    // 可变引用
    // ========================================================================
    println!("引用并修改 String：");
    // &mut 实际上是可以当作 & 传递的，所以注意以下要素
    // 要素一： s 本身必须是 mut
    // 要素二： 引用本身必须是 &mut
    let change = |s: &mut String| {
        s.push_str("world");
    };
    change(&mut s);
    println!("{:p} {:p} {} => s : {} ", &s, s.as_ptr(), s.len(), s);

    // 限制一：在特定作用域中的特定数据只能有一个可变引用，注意是可变引用
    {
        let r1 = &mut s;
        // let r2 = &mut s; // 声明两个变量指向同一引用导致编译警告，一旦使用（比如打印任一个）将导致编译错误
        println!("{:p} {:p} {} => r1 : {} ", &r1, r1.as_ptr(), r1.len(), r1);
    }
    let r2 = &mut s;
    println!("{:p} {:p} {} => r2 : {} ", &r2, r2.as_ptr(), r2.len(), r2);

    // 限制二：在特定作用域中不能同时使用可变与不可变引用
    // 多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{:p} {:p} {} => r1 : {} ", &r1, r1.as_ptr(), r1.len(), r1);
    println!("{:p} {:p} {} => r2 : {} ", &r2, r2.as_ptr(), r2.len(), r2);
    let r3 = &mut s; // 虽然声明不产生编译警告，但此后使用 r1 或 r2 将导致编译错误
    println!("{:p} {:p} {} => r3 : {} ", &r3, r3.as_ptr(), r3.len(), r3);
    // println!("{}, {}", r1, r2); // 编译错误：不能将 `s` 同时作为可变引用和不可变应用

    /*
    // ========================================================================
    // 悬垂引用（Dangling References）
    // ========================================================================
    在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer）
    所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
    相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：
        当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
    */
    println!("\n Rust编译器自动识别 悬垂引用（Dangling References）");
    // let r1 = dangle();
    let r2 = no_dangle();
    println!("{:p} {:p} {} => no_dangle : {} ", &r2, r2.as_ptr(), r2.len(), r2);
}
/*
// 返回字符串的引用  编译错误: 悬垂引用
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}// 这里 s 离开作用域并被丢弃。其内存被释放。
*/

// 直接返回 String
fn no_dangle() -> String {
    let s = String::from("hello");
    println!("{:p} {:p} {} => s : {} ", &s, s.as_ptr(), s.len(), s);

    s
} // 所有权被移动出去，所以没有值被释放。

fn calculate_references_length(s: &String) -> usize {
    println!("{} {:p} {:p} => references s", s, &s, s.as_ptr());
    s.len()
} // 因为不拥有引用值的所有权，离开作用域时其指向的值也不会被丢弃。
