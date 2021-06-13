// ========================================================================
// 避免引用循环：将 Rc<T> 变为 Weak<T>
// ========================================================================
/*
在26reference_counting.rs学习了引用计数：
调用 Rc::clone 会增加 Rc<T> 实例的 strong_count，和只在其 strong_count 为 0 时才会被清理的 Rc<T> 实例。
Rc<T>即强引用

接下来学习弱引用Weak<T>：
通过调用 Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用（weak reference）。
调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针。
不同于将 Rc<T> 实例的 strong_count 加1，调用 Rc::downgrade 会将 weak_count 加1。
Rc<T> 类型使用 weak_count 来记录其存在多少个 Weak<T> 引用，类似于 strong_count。
其区别在于 weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。

强引用代表如何共享 Rc<T> 实例的所有权，但弱引用并不属于所有权关系。
他们不会造成引用循环，因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。  <= 敲黑板...

因为 Weak<T> 引用的值可能已经被丢弃了，为了使用 Weak<T> 所指向的值，我们必须确保其值仍然有效。
为此可以调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。
如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None。
因为 upgrade 返回一个 Option<T>，我们确信 Rust 会处理 Some 和 None 的情况，所以它不会返回非法指针。

区别于前面学习的cons list例子只知道其下一项的list, 下面创建一个某项知道其子项和父项的树形结构的例子
老生长谈的树状数据结构，先知会一下概念：根节点root，叶子节点leaf，分支节点branch
 */

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    // 我们希望能够 Node 拥有其子节点，同时也希望通过变量来共享所有权，以便可以直接访问树中的每一个 Node，为此 Vec<T> 的项的类型被定义为 Rc<Node>。
    // 我们还希望能修改其他节点的子节点，所以 children 中 Vec<Rc<Node>> 被放进了 RefCell<T>。
    children: RefCell<Vec<Rc<Node>>>,

    /*
    为了使子节点知道其父节点，需要在 Node 结构体定义中增加一个 parent 字段。问题是 parent 的类型应该是什么。
    我们知道其不能包含 Rc<T>，因为这样 leaf.parent 将会指向 branch 而 branch.children 会包含 leaf 的指针，
    这会形成引用循环，会造成其 strong_count 永远也不会为 0.

    换一种方式思考这个关系，父节点应该拥有其子节点：如果父节点被丢弃了，其子节点也应该被丢弃。
    然而子节点不应该拥有其父节点：如果丢弃子节点，其父节点应该依然存在。
    这正是弱引用的例子！这样，一个节点就能够引用其父节点，但不拥有其父节点。
    */
    parent: RefCell<Weak<Node>>,
}


fn main() {
    // 创建一个leaf叶子节点：带有值 3 且没有子节点的 Node 实例
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    // leaf 开始时没有父节点，所以我们新建了一个空的 Weak 引用实例。
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    // 创建了一个新的内部作用域并将 branch 的创建放入其中，来观察 Rc<Node> 实例的 strong_count 和 weak_count 值的变化
    {
        // 创建一个branch分支节点：带有值 5 并以 leaf 作为子节点的实例
        let branch = Rc::new(Node {
            value: 5,
            // 这里克隆了 leaf 中的 Rc<Node> 并储存在了 branch 中，这意味着 leaf 中的 Node 现在有两个所有者：leaf和branch。
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });
        println!("进入内部作用域，branch被创建并关联leaf...");

        // 这里 leaf 的强引用计数为 2，因为现在 branch 的 branch.children 中储存了 leaf 的 Rc<Node> 的拷贝，不过弱引用计数仍然为 0。
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        // 此时 branch 中 Rc<Node> 的强引用计数为 1，弱引用计数为 0，因为leaf.parent还没有通过 Weak<Node> 指向 branch
        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch ), Rc::weak_count(&branch ));

        // 修改 leaf 使其拥有指向父节点的 Weak<Node> 引用。这里使用了 leaf 中 parent 字段里的 RefCell<Weak<Node>> 的 borrow_mut 方法，
        // 接着使用了 Rc::downgrade 函数来从 branch 中的 Rc<Node> 值创建了一个指向 branch 的 Weak<Node> 引用。
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf关联到branch...");
        // 因为 leaf.parent 通过 Weak<Node> 指向 branch，此时 branch 中 Rc<Node> 弱引用计数为 1
        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch ), Rc::weak_count(&branch ));

        // 当再次打印出 leaf 的父节点时，这一次将会得到存放了 branch 的 Some 值：现在 leaf 可以访问其父节点了！
        // 当leaf.parent打印其children及leaf本身时，我们也避免了如28cycle_reference.rs中最终会导致栈溢出的循环：Weak<Node> 引用被打印为 (Weak)：
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
    println!("离开内部作用域，branch被销毁...");
    // 当内部作用域结束时，branch 离开作用域，Rc<Node> 的强引用计数减少为 0，所以其 Node 被丢弃。
    // 来自 leaf.parent 的弱引用计数 1 与 Node 是否被丢弃无关，所以并没有产生任何内存泄漏！
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    // 在内部作用域结束后尝试访问 leaf 的父节点，会再次得到 None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 所有这些管理计数和值的逻辑都内建于 Rc<T> 和 Weak<T> 以及它们的 Drop trait 实现中。
    // 通过在 Node 定义中指定从子节点到父节点的关系为一个Weak<T>引用，就能够拥有父节点和子节点之间的双向引用而不会造成引用循环和内存泄漏。
}