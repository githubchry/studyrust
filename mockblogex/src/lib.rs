
// 思路： Post::new() => DraftPost::request_review() => PendingReviewPost::approve() => Post 完美闭环！

// 寥寥几行代码把面向对象的状态模式按在地上摩擦

// 定义 Post 并新建一个草案状态的实例
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        // 返回一个起草结构体
        DraftPost {
            content: String::new(), // 将 content 设置为新建的空 String。
        }
    }

    // 获取博文内容 注意 只有 Post 结构体才能调用！！
    pub fn content(&self) -> &str {
        &self.content
    }
}

// 现在不可能创建一个 Post 实例，因为 content 是私有的同时没有任何函数返回 Post。
// 如此现在程序确保了所有博文都从草案开始，同时草案博文没有任何可供展示的内容。任何绕过这些限制的尝试都会产生编译错误。


// 只有DraftPost 才可以修改博文内容, 起草博文可提交审核
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 实现状态转移为不同类型的转换
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

// 审核中博文 => 审核通过 => Post
//          => 审核不通过 => DraftPost
impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

}

/*
request_review 和 approve 方法获取 self 的所有权，因此会消费 DraftPost 和 PendingReviewPost 实例，
并分别转换为 PendingReviewPost 和发布的 Post。这样在调用 request_review 之后就不会遗留任何 DraftPost 实例，后者同理。

PendingReviewPost 并没有定义 content 方法，所以尝试读取其内容会导致编译错误，DraftPost 同理。

因为唯一得到定义了 content 方法的 Post 实例的途径是调用 PendingReviewPost 的 approve 方法，
而得到 PendingReviewPost 的唯一办法是调用 DraftPost 的 request_review 方法，现在我们就将发博文的工作流编码进了类型系统。

*/