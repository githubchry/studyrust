/*
内部可变性（Interior mutability）是 Rust 中的一个设计模式，
它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。
为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则。
当可以确保代码在运行时会遵守借用规则，即使编译器不能保证的情况，可以选择使用那些运用内部可变性模式的类型。
所涉及的 unsafe 代码将被封装进安全的 API 中，而外部类型仍然是不可变的。

Rust 通过其所有权机制，严格控制拥有和借用关系，来保证程序的安全，并且这种安全是在编译期可计算、可预测的。
但是这种严格的控制，有时也会带来灵活性的丧失，有的场景下甚至还满足不了需求。

因此，Rust 标准库中，设计了这样一个系统的组件：Cell, RefCell，它们弥补了 Rust 所有权机制在灵活性上和某些场景下的不足。
同时，又没有打破 Rust 的核心设计。它们的出现，使得 Rust 革命性的语言理论设计更加完整，更加实用。

具体是因为，它们提供了 内部可变性（相对于标准的 继承可变性 来讲的）。

通常，我们要修改一个对象，必须成为它的拥有者，并且声明 mut；或 以 &mut 的形式，借用；
而通过 Cell, RefCell，我们可以在需要的时候，就可以修改里面的对象。而不受编译期静态借用规则束缚。
Cell 有如下特点：
    Cell<T> 只能用于 T 实现了 Copy 的情况；
    .get()/.set()方法
```
    use std::cell::Cell;
    let c = Cell::new(5);
    let five = c.get();
    c.set(10);
```

相对于 Cell 只能包裹实现了 Copy 的类型，RefCell 用于更普遍的情况（其它情况都用 RefCell）。
相对于标准情况的 静态借用，RefCell 实现了 运行时借用，这个借用是临时的。
这意味着，编译器对 RefCell 中的内容，不会做静态借用检查，也意味着，出了什么问题，用户自己负责。
RefCell 的特点：
    在不确定一个对象是否实现了 Copy 时，直接选 RefCell；
    如果被包裹对象，同时被可变借用了两次，则会导致线程崩溃。所以需要用户自行判断；
    RefCell 只能用于线程内部，不能跨线程；
    RefCell 常常与 Rc 配合使用（都是单线程内部使用）；
*/

// 内部可变性的用例：mock 对象 => 见 mockobj 库项目 src/lib.rs => cargo test

// ========================================================================
// 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
// ========================================================================
/*
Rc<T> 允许对相同数据有多个所有者，不过只能提供数据的不可变访问。
如果有一个储存了 RefCell<T> 的 Rc<T> 的话，就可以得到有多个所有者 并且 可以修改的值了！

例如，回忆示例 26reference_counting.rs 的 cons list 的例子中使用 Rc<T> 使得多个列表共享另一个列表的所有权。
因为 Rc<T> 只存放不可变值，所以一旦创建了这些列表值后就不能修改。
让我们加入 RefCell<T> 来获得修改列表中值的能力。
*/

// 使用 use 语句将 Rc<T> 和 RefCell<T> 引入作用域，因为它不在 prelude 中。
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// 把List的两个值放到作用域就可以直接使用Cons和Nil，而不是List::Cons和List::Nil
use crate::List::{Cons, Nil};

fn main() {
    // 创建了一个 Rc<RefCell<i32>> 实例并储存在变量 value 中以便之后直接访问。
    let value = Rc::new(RefCell::new(5));

    // 接着在 a 中用包含 value 的 Cons 成员创建了一个 List。
    // 需要克隆 value 以便 a 和 value 都能拥有其内部值 5 的所有权，而不是将所有权从 value 移动到 a 或者让 a 借用 value。
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // 将 a 封装进了 Rc<T> 这样当创建列表 b 和 c 时，他们都可以引用 a
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // 将 value 的值加 10
    *value.borrow_mut() += 10;
    // *value 使用了`自动解引用`功能来解引用 Rc<T> 以获取其内部的 RefCell<T> 值。
    // borrow_mut 方法返回 RefMut<T> 智能指针，可以对其使用解引用运算符并修改其内部值。

    // 当我们打印出 a、b 和 c 时，可以看到他们都拥有修改后的值 15 而不是 5：
    println!("a list = {:?}", a);
    println!("b list = {:?}", b);
    println!("c list = {:?}", c);

    // 通过使用 RefCell<T>，我们可以拥有一个表面上不可变的 List，不过可以使用 RefCell<T> 中提供内部可变性的方法来在需要时修改数据。
    // RefCell<T> 的运行时借用规则检查也确实保护我们免于出现数据竞争——有时为了数据结构的灵活性而付出一些性能是值得的。
}
