// 定义 Post 并新建一个草案状态的实例
pub struct Post {
    // 在私有字段 state 中存放一个 Option<T> 类型的 trait 对象 Box<dyn State>
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        /*
        当创建新的 Post 时，我们将其 state 字段设置为一个存放了 Box 的 Some 值。这个 Box 指向一个 Draft 结构体新实例。
        这确保了无论何时新建一个 Post 实例，它都会从草案开始。
        因为 Post 的 state 字段是私有的，也就无法创建任何其他状态的 Post 了！
        */
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(), // 将 content 设置为新建的空 String。
        }
    }

    // 写入博文内容
    pub fn add_text(&mut self, text: &str) {
        // add_text 获取一个 self 的可变引用，因为需要改变调用 add_text 的 Post 实例
        self.content.push_str(text);    // push_str追加字符串text到content后面
    }

    // 获取博文内容
    pub fn content(&self) -> &str {
        // 如果状态为 Published 希望返回博文 content 字段的值；否则希望返回空字符串 slice
        self.state.as_ref().unwrap().content(self)
        // 这里调用 Option 的 as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权。
        // 因为 state 是一个 Option<Box<State>>，调用 as_ref 会返回一个 Option<&Box<State>>。
        // 如果不调用 as_ref，将会得到一个错误，因为不能将 state 移动出借用的 &self 函数参数。
        // 接着调用 unwrap 方法，这里我们知道它永远也不会 panic，因为 Post 的所有方法都确保在他们返回时 state 会有一个 Some 值。
        // 接着我们就有了一个 &Box<State>，当调用其 content 时，解引用强制多态会作用于 & 和 Box ，
        // 这样最终会调用实现了 State trait 的类型的 content 方法。这意味着需要为 State trait 定义增加 content，
        // 这也是放置根据所处状态返回什么内容的逻辑的地方
    }

    // 请求审核博文来改变其状态
    pub fn request_review(&mut self) {
        // Post 的 state 字段中 Option 的作用：调用 take 方法将 state 字段中的 Some 值取出并留下一个 None，
        // 因为 Rust 不允许在结构体中存在空的字段。这使得我们将 state 值移动出 Post 而不是借用它。
        // 接着将博文的 state 值设置为这个操作的结果。
        if let Some(s) = self.state.take() {
            // 此时state为None！骚操作！！
            // 此时s为state原来的状态！骚操作！！给跪了！！

            // 这里将 state 临时设置为 None，
            // 不同于像 self.state = self.state.request_review(); 这样的代码直接设置 state 字段，来获取 state 值的所有权。
            // 这确保了当 Post 被转换为新状态后其不再能使用老的 state 值。贼细节


            // 在 Post 的当前状态下调用内部的 request_review 方法: 会消费当前的状态并返回一个新状态。
            // 接着将博文的 state 值设置为这个新状态。即更新状态...
            self.state = Some(s.request_review())
        }
    }

    // 将 state 设置为审核通过时应处于的状态
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// State trait 定义了所有不同状态的博文所共享的行为，同时 Draft、PendingReview 和 Published 状态都会实现 State 状态。
trait State {
    /*
    所有实现了这个 trait 的类型现在都需要实现 request_review 方法。
    注意不同于使用 self、 &self 或者 &mut self 作为方法的第一个参数，这里使用了 self: Box<Self>。
    这个语法意味着这个方法调用只对这个类型的 Box 有效。（=> Draft、PendingReview 和 Published）
    这个语法获取了 Box<Self> 的所有权，使老状态无效化以便 Post 的状态值可以将自身转换为新状态。
    */

    // 请求审核
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // 审核通过
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // 返回内容：默认空，仅需在Published状态重载返回内容
    // 注意这个方法需要生命周期注解: 这里获取 post 的引用作为参数，并返回 post 一部分的引用（Published），所以返回的引用的生命周期与 post 参数相关。
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// 草案状态
struct Draft {}

impl State for Draft {
    // 请求审核博文来改变其状态: 将其状态由 Draft 改为 PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// 等待审核状态
struct PendingReview {}

impl State for PendingReview {
    // 返回自身，不进行任何状态转换。因为请求审核已经处于 PendingReview 状态的博文应该保持 PendingReview 状态。
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // 返回一个新的、装箱的 Published 结构体的实例 表示审核通过，已发布状态
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// 已发布状态
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

