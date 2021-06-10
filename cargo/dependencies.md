

## 依赖关系

```
# dependencies 片段告诉 Cargo 本项目依赖了哪些外部 crate 及其版本
[dependencies]
# Rust 标准库中尚未包含随机数功能，但提供了一个 rand crate
# 语义化版本 0.8.3  事实上是 ^0.8.3 的简写，它表示 “任何与 0.8.3 版本公有 API 相兼容的版本”。
rand = "0.5.5"  #引入一个 rand 依赖

```

现在我们有了一个外部依赖，Cargo 从 *registry* 上获取所有包的最新版本信息，这是一份来自 [Crates.io](https://crates.io/) 的数据拷贝。Crates.io 是 Rust 生态环境中的开发者们向他人贡献 Rust 开源项目的地方。

在更新完 registry 后，Cargo 检查 `[dependencies]` 片段并下载缺失的 crate 。本例中，虽然只声明了 `rand` 一个依赖，然而 Cargo 还是额外获取了 `libc` 和 `rand_core` 的拷贝，因为 `rand` 依赖 `libc` 和 `rand_core` 来正常工作。下载完成后，Rust 编译依赖，然后使用这些依赖编译项目。

如果不做任何修改，立刻再次运行 `cargo build`，则不会看到任何除了 `Finished` 行之外的输出。Cargo 知道它已经下载并编译了依赖，同时 *Cargo.toml* 文件也没有变动。Cargo 还知道代码也没有任何修改，所以它不会重新编译代码。因为无事可做，它简单的退出了。

#### Cargo.lock 文件确保构建是可重现的

Cargo 有一个机制来确保任何人在任何时候重新构建代码，都会产生相同的结果：Cargo 只会使用你指定的依赖版本，除非你又手动指定了别的。例如，如果下周 `rand` crate 的 `0.5.6` 版本出来了，它修复了一个重要的 bug，同时也含有一个会破坏代码运行的缺陷，这时会发生什么呢？

这个问题的答案是 *Cargo.lock* 文件。它在第一次运行 `cargo build` 时创建，并放在 *guessing_game* 目录。当第一次构建项目时，Cargo 计算出所有符合要求的依赖版本并写入 *Cargo.lock* 文件。当将来构建项目时，Cargo 会发现 *Cargo.lock* 已存在并使用其中指定的版本，而不是再次计算所有的版本。这使得你拥有了一个自动化的可重现的构建。换句话说，项目会持续使用 `0.5.5` 直到你显式升级，多亏有了 *Cargo.lock* 文件。

#### 更新 crate 到一个新版本

当你 **确实** 需要升级 crate 时，Cargo 提供了另一个命令，`update`，它会忽略 *Cargo.lock* 文件，并计算出所有符合 *Cargo.toml* 声明的最新版本。如果成功了，Cargo 会把这些版本写入 *Cargo.lock* 文件。

不过，Cargo 默认只会寻找大于 `0.5.5` 而小于 `0.6.0` 的版本。如果 `rand` crate 发布了两个新版本，`0.5.6` 和 `0.6.0`，在运行 `cargo update` 时会出现如下内容：

```
$ cargo update
    Updating crates.io index
    Updating rand v0.5.5 -> v0.5.6
```

这时，你也会注意到的 *Cargo.lock* 文件中的变化无外乎现在使用的 `rand` crate 版本是`0.5.6`

如果想要使用 `0.6.0` 版本的 `rand` 或是任何 `0.6.x` 系列的版本，必须像这样更新 *Cargo.toml* 文件：

```
[dependencies]

rand = "0.6.0"
```

下一次运行 `cargo build` 时，Cargo 会从 registry 更新可用的 crate，并根据你指定的新版本重新计算。

#### 程序中使用 rand 依赖库

```
use rand::Rng;
let secret_number = rand::thread_rng().gen_range(1, 101);
```

首先，我们新增了一行 `use`：`use rand::Rng`。`Rng` 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中。第十章会详细介绍 trait。

接下来，我们在中间还新增加了两行。`rand::thread_rng` 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。接下来，调用随机数生成器的 `gen_range` 方法。这个方法由刚才引入到作用域的 `Rng` trait 定义。`gen_range` 方法获取两个数字作为参数，并生成一个范围在两者之间的随机数。它包含下限但不包含上限，所以需要指定 `1` 和 `101` 来请求一个 1 和 100 之间的数。

> 注意：你不可能凭空就知道应该 use 哪个 trait 以及该从 crate 中调用哪个方法。crate 的使用说明位于其文档中。Cargo 有一个很棒的功能是：运行 `cargo doc --open` 命令来构建所有本地依赖提供的文档，并在浏览器中打开。例如，假设你对 `rand` crate 中的其他功能感兴趣，你可以运行 `cargo doc --open` 并点击左侧导航栏中的 `rand`。

