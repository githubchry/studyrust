
// ========================================================================
// 引用循环与内存泄漏
// ========================================================================
/*
Rust 的内存安全性保证使其难以意外地制造永远也不会被清理的内存（被称为 内存泄漏（memory leak）），但并不是不可能。
与在编译时拒绝数据竞争不同， Rust 并不保证完全地避免内存泄漏，这意味着内存泄漏在 Rust 被认为是内存安全的。
这一点可以通过 Rc<T> 和 RefCell<T> 看出：创建引用循环的可能性是存在的。
这会造成内存泄漏，因为每一项的引用计数永远也到不了 0，其值也永远不会被丢弃。

下面制造一个引用循环：
 */
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    // RefCell<Rc<List>>，这意味着不同于像 27refcell.rs 那样能够修改 i32 的值，我们希望能够修改 Cons 成员所指向的 List。
    Nil,
}

impl List {
    // 增加了一个 tail 方法来方便我们在有 Cons 成员的时候访问其第二项。
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // 创建一个指向a的list
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    /*
          ┌───────┐              ┌───────┐
          │   a   │              │   b   │
          └───┬───┘              └───┬───┘
              │                      │
        ┌─────↓─┬───┐   ┌───┐  ┌─────↓─┬───┐
        │   5   │   │─→ │Nil│  │   10  │   │
        └─↑─────┴───┘   └───┘  └───────┴─┬─┘
          │                              │
          └──────────────────────────────┘
    */

    // 修改 list a, 让 list a 指向 list b 中 => 循环引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    /*
          ┌───────┐              ┌───────┐
          │   a   │              │   b   │
          └───┬───┘              └───┬───┘
              │                      │
        ┌─────↓─┬───┐          ┌─────↓─┬───┐
        │   5   │   │─────────→│   10  │   │
        └─↑─────┴───┘          └───────┴─┬─┘
          │                              │
          └──────────────────────────────┘
    */

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    println!("a next item = {:?}", a.tail()); // Rust 会尝试打印出 a 指向 b 指向 a 这样的循环直到栈溢出。
}
/*
如果你有包含 Rc<T> 的 RefCell<T> 值或类似的嵌套结合了内部可变性和引用计数的类型，请务必小心确保你没有形成一个引用循环；你无法指望 Rust 帮你捕获它们。
创建引用循环是一个程序上的逻辑 bug，你应该使用自动化测试、代码评审和其他软件开发最佳实践来使其最小化。

另一个解决方案是重新组织数据结构，使得一部分引用拥有所有权而另一部分没有。
换句话说，循环将由一些拥有所有权的关系和一些无所有权的关系组成，只有所有权关系才能影响值是否可以被丢弃。
    29weak_reference.rs —— 引入弱循环
*/