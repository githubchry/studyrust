/*
// ========================================================================
// 内部可变性的用例：mock 对象
// ========================================================================
测试替身（test double）是一个通用编程概念，它代表一个在测试中替代某个类型的类型。
mock 对象 是特定类型的测试替身，它们记录测试过程中发生了什么以便可以断言操作是正确的。

虽然 Rust 中的对象与其他语言中的对象并不是一回事，Rust 也没有像其他语言那样在标准库中内建 mock 对象功能，
不过我们确实可以创建一个与 mock 对象有着相同功能的结构体。

如下是一个我们想要测试的场景：
我们在编写一个记录某个值与最大值的差距的库，并根据当前值与最大值的差距来发送消息。
该库只提供记录与最大值的差距，以及何种情况发送什么消息的功能。

使用此库的程序则期望提供实际发送消息的机制：
    程序可以选择记录一条消息、发送 email、发送短信等等。
    库本身无需知道这些细节；只需实现其提供的 Messenger trait 即可。
*/

// 拥有一个方法 send 的 Messenger trait，是我们的 mock 对象所需要拥有的接口，其获取一个 self 的不可变引用和文本信息。
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;


    // 我们所需的 mock 对象是，调用 send 并不实际发送 email 或消息，而是只记录信息被通知要发送了。
    struct MockMessenger {
        sent_messages_raw: Vec<String>,
        // 错误示范：定义一个容器来记录信息
        sent_messages_refcell: RefCell<Vec<String>>,// 正确示范：定义一个RefCell容器来记录信息
    }

    // 定义了一个关联函数 new 以便于新建从空消息列表开始的 MockMessenger 值
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages_raw: vec![],
                sent_messages_refcell: RefCell::new(vec![]),
            }
        }
    }

    // 接着为 MockMessenger 实现 Messenger trait 这样就可以为 LimitTracker 提供一个 MockMessenger。
    impl Messenger for MockMessenger {
        // 获取传入的消息作为参数并储存在 MockMessenger 的 sent_messages 列表中。
        fn send(&self, message: &str) {
            // 这是假设我们去使用别人已经设计好的库的情况下，我们不能去修改别人trait签名，只能去适应....
            // 编译错误: 无法修改 MockMessenger 来记录消息，因为 send 方法获取了 self 的不可变引用。
            // self.sent_messages_raw.push(String::from(message)); // 编译错误

            // 我们也不能参考错误文本的建议使用 &mut self 替代，因为这样 send 的签名就不符合 Messenger trait 定义中的签名了
            // 这正是内部可变性的用武之地！我们将通过 RefCell 来储存 sent_messages_refcell，
            // 然后 send 将能够修改 sent_messages_refcell 并储存消息。
            self.sent_messages_refcell.borrow_mut().push(String::from(message));
            // 调用 self.sent_messages_refcell 中 RefCell 的 borrow_mut 方法来获取 RefCell 中值的可变引用，这是一个 vector。
            // 接着可以对 vector 的可变引用调用 push 以便记录测试过程中看到的消息。

            //故意尝试在相同作用域创建两个可变借用以便演示 RefCell<T> 不允许我们在运行时这么做：
            // let mut one_borrow = self.sent_messages_refcell.borrow_mut();
            // let mut two_borrow = self.sent_messages_refcell.borrow_mut();

        }
    }

    // 测试当 LimitTracker 被告知将 value 设置为超过 max 值 75% 的某个值
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // 首先新建一个 MockMessenger，其从空消息列表开始。
        let mock_messenger = MockMessenger::new();
        // 接着新建一个 LimitTracker 并传递新建 MockMessenger 的引用和 max 值 100。
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        // 我们使用值 80 调用 LimitTracker 的 set_value 方法，这超过了 100 的 75%。
        limit_tracker.set_value(80);
        // 断言 MockMessenger 中记录的消息列表应该有一条消息
        // assert_eq!(mock_messenger.sent_messages_raw.len(), 1);
        assert_eq!(mock_messenger.sent_messages_refcell.borrow().len(), 1);
        // 为了看到sent_messages_refcell内部 vector 中有多少个项，需要调用 RefCell 的 borrow 以获取 vector 的不可变引用。
    }
}

/*
现在我们见识了如何使用 RefCell<T>，让我们研究一下它怎样工作的！
// ========================================================================
// RefCell<T> 在运行时记录借用
// ========================================================================
当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。
对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法，这属于 RefCell<T> 安全 API 的一部分。
borrow 方法返回 Ref<T> 类型的智能指针，borrow_mut 方法返回 RefMut 类型的智能指针。
这两个类型都实现了 Deref，所以可以当作常规引用对待。

RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。
每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。当 Ref<T> 值离开作用域时，不可变借用计数减一。
就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。
如果我们尝试违反这些规则，相比引用时的编译时错误，RefCell<T> 的实现会在运行时出现 panic。

为了看到sent_messages_refcell内部 vector 中有多少个项，需要调用 RefCell 的 borrow 以获取 vector 的不可变引用。

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}

这里为 borrow_mut 返回的 RefMut 智能指针创建了 one_borrow 变量。
接着用相同的方式在变量 two_borrow 创建了另一个可变借用。
这会在相同作用域中创建两个可变引用，这是不允许的。
当运行库的测试时，编译时不会有任何错误，不过测试会失败

在运行时捕获借用错误而不是编译时意味着将会在开发过程的后期才会发现错误，甚至有可能发布到生产环境才发现；
还会因为在运行时而不是编译时记录借用而导致少量的运行时性能惩罚。
然而，使用 RefCell 使得在只允许不可变值的上下文中编写修改自身以记录消息的 mock 对象成为可能。
虽然有取舍，但是我们可以选择使用 RefCell<T> 来获得比常规引用所能提供的更多的功能。

结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者 => 见27refcell.rs
*/
