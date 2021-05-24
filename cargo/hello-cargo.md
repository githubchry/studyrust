## Hello, Cargo!

Cargo 是 Rust 的构建系统和包管理器。大多数 Rustacean 们使用 Cargo 来管理他们的 Rust 项目，因为它可以为你处理很多任务，比如构建代码、下载依赖库并编译这些库。（我们把代码所需要的库叫做 **依赖**（*dependencies*））。

最简单的 Rust 程序，比如我们刚刚编写的，没有任何依赖。所以如果使用 Cargo 来构建 “Hello, world!” 项目，将只会用到 Cargo 构建代码的那部分功能。在编写更复杂的 Rust 程序时，你将添加依赖项，如果使用 Cargo 启动项目，则添加依赖项将更容易。

由于绝大多数 Rust 项目使用 Cargo，本书接下来的部分假设你也使用 Cargo。如果使用 [“安装”](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html#installation) 部分介绍的官方安装包的话，则自带了 Cargo。如果通过其他方式安装的话，可以在终端输入如下命令检查是否安装了 Cargo：

```text
$ cargo --version
```

如果你看到了版本号，说明已安装！如果看到类似 `command not found` 的错误，你应该查看相应安装文档以确定如何单独安装 Cargo。

### 相关概念

#### package 和 crate

crate 是一个二进制项(*binary crate*)或者库(*library crate*)。*crate root* 是一个源文件，Rust 编译器以它为起始点，并构成 crate 的根模块。

包（package）是提供一系列功能的一个或者多个 crate。

一个包会包含有一个 *Cargo.toml* 文件，阐述如何去构建这些 crate。

create分为  和 *library crate* ，每个create都有一个 *crate root*  ，

规则：

- 包中至多 **只能** 包含一个库 crate(library crate)；

- 包中可以包含任意多个二进制 crate(binary crate)；

- 包中至少包含一个 crate，无论是库的还是二进制的。

约定:

- *src/main.rs* 就是一个与包同名的二进制 crate 的 crate 根
- 如果包目录中包含 *src/lib.rs*，则包带有与其同名的库 crate，且 *src/lib.rs* 是 crate 根。



一个 crate 会将一个作用域内的相关功能分组到一起，使得该功能可以很方便地在多个项目之间共享。

#### 模块

- binary crate - 二进制项

- library crate - 库

- *crate root*

  *crate root* 是一个源文件，Rust 编译器以它为起始点，并构成 crate 的根模块。

- 模块

### 使用 Cargo 创建项目（包）

```text
$ cargo new hello_cargo
$ cd hello_cargo
```

第一行命令新建了名为 *hello_cargo* 的目录。我们将项目命名为 *hello_cargo*，同时 Cargo 在一个同名目录中创建项目文件。

进入 *hello_cargo* 目录并列出文件。将会看到 Cargo 生成了两个文件和一个目录：一个 *Cargo.toml* 文件，一个 *src* 目录，以及位于 *src* 目录中的 *main.rs* 文件。它也在 *hello_cargo* 目录初始化了一个 git 仓库，以及一个 *.gitignore* 文件。

> Cargo 遵循的一个约定：*src/main.rs* 就是一个与包同名的二进制 crate 的 crate 根。同样的，Cargo 知道如果包目录中包含 *src/lib.rs*，则包带有与其同名的库 crate，且 *src/lib.rs* 是 crate 根。crate 根文件将由 Cargo 传递给 `rustc` 来实际构建库或者二进制项目。

请自行选用文本编辑器打开 *Cargo.toml* 文件。它应该看起来如示例 1-2 所示：

文件名: Cargo.toml

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

示例 1-2: *cargo new* 命令生成的 *Cargo.toml* 的内容

这个文件使用 [*TOML*](https://github.com/toml-lang/toml) (*Tom's Obvious, Minimal Language*) 格式，这是 Cargo 配置文件的格式。

第一行，`[package]`，是一个片段（section）标题，表明下面的语句用来配置一个包。随着我们在这个文件增加更多的信息，还将增加其他片段（section）。

接下来的四行设置了 Cargo 编译程序所需的配置：项目的名称、版本、作者以及要使用的 Rust 版本。Cargo 从环境中获取你的名字和 email 信息，所以如果这些信息不正确，请修改并保存此文件。附录 E 会介绍 `edition` 的值。

最后一行，`[dependencies]`，是罗列项目依赖的片段的开始。在 Rust 中，代码包被称为 *crates*。这个项目并不需要其他的 crate，不过在第二章的第一个项目会用到依赖，那时会用得上这个片段。

现在打开 *src/main.rs* 看看：

文件名: src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo 为你生成了一个 “Hello, world!” 程序，Cargo 将代码放在 *src* 目录，同时项目根目录包含一个 *Cargo.toml* 配置文件。

Cargo 期望源文件存放在 *src* 目录中。项目根目录只存放 README、license 信息、配置文件和其他跟代码无关的文件。使用 Cargo 帮助你保持项目干净整洁，一切井井有条。

如果没有使用 Cargo 开始项目，比如我们创建的 Hello,world! 项目，可以将其转化为一个 Cargo 项目。将代码放入 *src* 目录，并创建一个合适的 *Cargo.toml* 文件。

### 构建并运行 Cargo 项目

现在让我们看看通过 Cargo 构建和运行 “Hello, world!” 程序有什么不同！在 *hello_cargo* 目录下，输入下面的命令来构建项目：

```text
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

这个命令会创建一个可执行文件 *target/debug/hello_cargo* （在 Windows 上是 *target\debug\hello_cargo.exe*），而不是放在目前目录下。可以通过这个命令运行可执行文件：

```text
$ ./target/debug/hello_cargo # 或者在 Windows 下为 .\target\debug\hello_cargo.exe
Hello, world!
```

如果一切顺利，终端上应该会打印出 `Hello, world!`。首次运行 `cargo build` 时，也会使 Cargo 在项目根目录创建一个新文件：*Cargo.lock*。这个文件记录项目依赖的实际版本。这个项目并没有依赖，所以其内容比较少。你自己永远也不需要碰这个文件，让 Cargo 处理它就行了。

我们刚刚使用 `cargo build` 构建了项目，并使用 `./target/debug/hello_cargo` 运行了程序，也可以使用 `cargo run` 在一个命令中同时编译并运行生成的可执行文件：

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

注意这一次并没有出现表明 Cargo 正在编译 `hello_cargo` 的输出。Cargo 发现文件并没有被改变，就直接运行了二进制文件。如果修改了源文件的话，Cargo 会在运行之前重新构建项目，并会出现像这样的输出：

```text
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo 还提供了一个叫 `cargo check` 的命令。该命令快速检查代码确保其可以编译，但并不产生可执行文件：

```text
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

为什么你会不需要可执行文件呢？通常 `cargo check` 要比 `cargo build` 快得多，因为它省略了生成可执行文件的步骤。如果你在编写代码时持续的进行检查，`cargo check` 会加速开发！为此很多 Rustaceans 编写代码时定期运行 `cargo check` 确保它们可以编译。当准备好使用可执行文件时才运行 `cargo build`。

我们回顾下已学习的 Cargo 内容：

- 可以使用 `cargo build` 或 `cargo check` 构建项目。
- 可以使用 `cargo run` 一步构建并运行项目。
- 有别于将构建结果放在与源码相同的目录，Cargo 会将其放到 *target/debug* 目录。

使用 Cargo 的一个额外的优点是，不管你使用什么操作系统，其命令都是一样的。所以从现在开始本书将不再为 Linux 和 macOS 以及 Windows 提供相应的命令。

### 发布（release）构建

当项目最终准备好发布时，可以使用 `cargo build --release` 来优化编译项目。这会在 *target/release* 而不是 *target/debug* 下生成可执行文件。这些优化可以让 Rust 代码运行的更快，不过启用这些优化也需要消耗更长的编译时间。这也就是为什么会有两种不同的配置：一种是为了开发，你需要经常快速重新构建；另一种是为用户构建最终程序，它们不会经常重新构建，并且希望程序运行得越快越好。如果你在测试代码的运行时间，请确保运行 `cargo build --release` 并使用 *target/release* 下的可执行文件进行测试。

### 把 Cargo 当作习惯

对于简单项目， Cargo 并不比 `rustc` 提供了更多的优势，不过随着开发的深入，终将证明其价值。对于拥有多个 crate 的复杂项目，交给 Cargo 来协调构建将简单的多。

即便 `hello_cargo` 项目十分简单，它现在也使用了很多在你之后的 Rust 生涯将会用到的实用工具。其实，要在任何已存在的项目上工作时，可以使用如下命令通过 Git 检出代码，移动到该项目目录并构建：

```text
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

关于更多 Cargo 的信息，请查阅 [其文档](https://doc.rust-lang.org/cargo/)。

## 使用Cargo 编译多个二进制文件

添加`[[bin]]`节点，每个节点对应一个二进制文件

```
[package]
name = "studyrust"
version = "0.1.0"
authors = ["chry <a8512413@163.com>"]
edition = "2018"

[[bin]]
name = "01hello_world"
path = "src/01hello_world.rs"

[[bin]]
name = "02variable_const"
path = "src/02variable_const.rs"

[[bin]]
name = "03data_type"
path = "src/03data_type.rs"

[[bin]]
name = "04function"
path = "src/04function.rs"

[[bin]]
name = "05control_flow"
path = "src/05control_flow.rs"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```





# 依赖关系

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
