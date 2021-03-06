/*
实现一个增量式的发布博文的工作流。这个博客的最终功能看起来像这样：
    1.博文从空白的草案开始。
    2.一旦草案完成，请求审核博文。
    3.一旦博文过审，它将被发表。
    4.只有被发表的博文的内容会被打印，这样就不会意外打印出没有被审核的博文的文本。

任何其他对博文的修改尝试都是没有作用的。例如，如果尝试在请求审核之前通过一个草案博文，博文应该保持未发布的状态。


与 crate 交互的唯一的类型是 Post。这个类型会使用状态模式并会存放处于三种博文所可能的状态之一的值 —— 草案，等待审核和发布。
状态上的改变由 Post 类型内部进行管理。状态依库用户对 Post 实例调用的方法而改变，但是不能直接管理状态变化。
这也意味着用户不会在状态上犯错，比如在过审前发布博文。
*/

use mockblog::Post;

fn main() {
    // 创建一个新的博文草案
    let mut post = Post::new();

    // 在草案阶段为博文编写一些文本
    post.add_text("Hello");
    assert_eq!("", post.content());

    post.add_text(" ");

    // 尝试在审核之前立即打印出博文的内容，什么也不会发生因为博文仍然是草案
    post.request_review();
    assert_eq!("", post.content()); // 出于演示目的这里增加的 assert_eq!

    post.add_text("---");   // 审核阶段，修改不起作用

    // 审核不通过 => 回退到草案阶段
    post.reject();

    post.add_text("World");

    post.request_review();

    // 审核通过，而在等待审核的阶段 content 应该仍然返回空字符串
    post.approve();
    // 最后当博文审核通过，它应该被发表，这意味着当调用 content 时博文的文本将被返回。
    assert_eq!("Hello World", post.content());
}
