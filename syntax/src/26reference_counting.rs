// ========================================================================
// Rc<T> 引用计数智能指针
// ========================================================================
/*
大部分情况下所有权是非常明确的：可以准确地知道哪个变量拥有某个值。然而，有些情况单个值可能会有多个所有者。
例如，在图数据结构中，多个边可能指向相同的节点，而这个节点从概念上讲为所有指向它的边所拥有。节点直到没有任何边指向它之前都不应该被清理。

为了启用多所有权，Rust 有一个叫做 Rc<T> 的类型。其名称为 引用计数（reference counting）的缩写。
引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。

可以将其想象为客厅中的电视。当一个人进来看电视时，他打开电视。其他人也可以进来看电视。
当最后一个人离开房间时，他关掉电视因为它不再被使用了。如果某人在其他人还在看的时候就关掉了电视，正在看电视的人肯定会抓狂的！

Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候。
如果确实知道哪部分是最后一个结束使用的话，就可以令其成为数据的所有者，正常的所有权规则就可以在编译时生效。

注意 Rc<T> 只能用于单线程场景；第十六章并发会涉及到如何在多线程程序中进行引用计数。


// 以23box_list.rs里面设计的cons list作为例子, 创建一个列表a，假设希望创建两个共享列表b和c来所有权的列表：
    let a = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let b = Cons(3, Box::new(a));   // b list 意图：以3为开头，后面接上a
    let c = Cons(4, Box::new(a));   // c list 意图：以4为开头，后面接上a
上面的代码意图明显，但是会产生编译错误：
    Cons 成员拥有其储存的数据，所以当创建 b 列表时，a 被移动进了 b 这样 b 就拥有了 a。
    接着当再次尝试使用 a 创建 c 时，这不被允许，因为 a 的所有权已经被移动。

可以改变 Cons 的定义来存放一个引用，不过接着必须指定生命周期参数。
通过指定生命周期参数，表明列表中的每一个元素都至少与列表本身存在的一样久。
例如，借用检查器不会允许 let a = Cons(10, &Nil); 编译，因为临时值 Nil 会在 a 获取其引用之前就被丢弃了。

相反，我们修改 List 的定义为使用 Rc<T> 代替 Box<T>:
*/

// 需要使用 use 语句将 Rc<T> 引入作用域，因为它不在 prelude 中。
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    // 现在每一个 Cons 变量都包含一个值和一个指向 List 的 Rc<T>。
    Nil,
}

// 把List的两个值放到作用域就可以直接使用Cons和Nil，而不是List::Cons和List::Nil
use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // 调用 Rc::clone 函数并传递 a 中 Rc<List> 的引用作为参数。
    // 也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    /*
    当创建 b 时，不同于获取 a 的所有权，这里会克隆 a 所包含的 Rc<List>，这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc<List> 中数据的所有权。
    创建 c 时也会克隆 a，这会将引用计数从 2 增加为 3。
    每次调用 Rc::clone，Rc<List> 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理。

    Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。
    Rc::clone 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。
    通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。
    当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
    */
    println!("a list = {:?}", a);
    println!("b list = {:?}", b);
    println!("c list = {:?}", c);

    // ========================================================================
    // 通过 Rc::strong_count 方法查看引用计数
    // ========================================================================

    let x = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));  // 初始引用计数为1
    println!("count after creating x = {}", Rc::strong_count(&x));
    let y = Cons(3, Rc::clone(&x));     // 每次调用 clone，计数会增加1
    println!("count after creating y = {}", Rc::strong_count(&x));
    {
        let z = Cons(4, Rc::clone(&x)); // 每次调用 clone，计数会增加1
        println!("count after creating z = {}", Rc::strong_count(&x));
    }   // 当 z 离开作用域时，计数减1。
    println!("count after z goes out of scope = {}", Rc::strong_count(&x));
    /*
    从这个例子我们所不能看到的是，在 main 的结尾当 b 然后是 a 离开作用域时，此处计数会是 0，同时 Rc<List> 被完全清理。
    使用 Rc<T> 允许一个值有多个所有者，引用计数则确保只要任何所有者依然存在其值也保持有效。
    */
}
