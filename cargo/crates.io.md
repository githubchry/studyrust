## [将 crate 发布到 Crates.io](https://rustwiki.org/zh-CN/book/ch14-02-publishing-to-crates-io.html#将-crate-发布到-cratesio)

我们曾经在项目中使用 [crates.io](https://crates.io/) 上的包作为依赖，不过你也可以通过发布自己的包来向它人分享代码。[crates.io](https://crates.io/) 用来分发包的源代码，所以它主要托管开源代码。

Rust 和 Cargo 有一些帮助它人更方便找到和使用你发布的包的功能。我们将介绍一些这样的功能，接着讲到如何发布一个包。

### [创建 Crates.io 账号](https://rustwiki.org/zh-CN/book/ch14-02-publishing-to-crates-io.html#创建-cratesio-账号)

在你可以发布任何 crate 之前，需要在 [crates.io](https://crates.io/) 上注册账号并获取一个 API token。为此，访问位于 [crates.io](https://crates.io/) 的首页并使用 GitHub 账号登陆。（目前 GitHub 账号是必须的，不过将来该网站可能会支持其他创建账号的方法）一旦登陆之后，查看位于 https://crates.io/me/ 的账户设置页面并获取 API token。接着使用该 API token 运行 `cargo login` 命令，像这样：

```text
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

这个命令会通知 Cargo 你的 API token 并将其储存在本地的 *~/.cargo/credentials* 文件中。注意这个 token 是一个 **秘密**（**secret**）且不应该与其他人共享。如果因为任何原因与他人共享了这个信息，应该立即到 [crates.io](https://crates.io/) 重新生成这个 token。

### [发布新 crate 之前](https://rustwiki.org/zh-CN/book/ch14-02-publishing-to-crates-io.html#发布新-crate-之前)

有了账号之后，比如说你已经有一个希望发布的 crate。在发布之前，你需要在 crate 的 *Cargo.toml* 文件的 `[package]` 部分增加一些本 crate 的元信息（metadata）。

首先 crate 需要一个唯一的名称。虽然在本地开发 crate 时，可以使用任何你喜欢的名称。不过 [crates.io](https://crates.io/) 上的 crate 名称遵守先到先得的分配原则。一旦某个 crate 名称被使用，其他人就不能再发布这个名称的 crate 了。请在网站上搜索你希望使用的名称来找出它是否已被使用。如果没有，修改 *Cargo.toml* 中 `[package]` 里的名称为你希望用于发布的名称，像这样：

文件名: Cargo.toml

```toml
[package]
name = "chry_minigrep"
```

即使你选择了一个唯一的名称，如果此时尝试运行 `cargo publish` 发布该 crate 的话，会得到一个警告接着是一个错误：

```text
$ cargo publish
    Updating registry `https://github.com/rust-lang/crates.io-index`
warning: manifest has no description, license, license-file, documentation,
homepage or repository.
--snip--
error: api errors: missing or empty metadata fields: description, license.
```

这是因为我们缺少一些关键信息：关于该 crate 用途的描述和用户可能在何种条款下使用该 crate 的 license。为了修正这个错误，需要在 *Cargo.toml* 中引入这些信息。

描述通常是一两句话，因为它会出现在 crate 的搜索结果中和 crate 页面里。对于 `license` 字段，你需要一个 **license 标识符值**（*license identifier value*）。[Linux 基金会的 Software Package Data Exchange (SPDX)](https://spdx.org/licenses/) 列出了可以使用的标识符。例如，为了指定 crate 使用 MIT License，增加 `MIT` 标识符：

文件名: Cargo.toml

```toml
[package]
name = "chry_minigrep"
license = "MulanPSL-2.0"
```

如果你希望使用不存在于 SPDX 的 license，则需要将 license 文本放入一个文件，将该文件包含进项目中，接着使用 `license-file` 来指定文件名而不是使用 `license` 字段。

关于项目所适用的 license 指导超出了本书的范畴。很多 Rust 社区成员选择与 Rust 自身相同的 license，这是一个双许可的 `MIT OR Apache-2.0`。这个实践展示了也可以通过 `OR` 分隔为项目指定多个 license 标识符。

那么，有了唯一的名称、版本号、由 `cargo new` 新建项目时增加的作者信息、描述和所选择的 license，已经准备好发布的项目的 *Cargo.toml* 文件可能看起来像这样：

文件名: Cargo.toml

```toml
[package]
name = "chry_minigrep"  # 如果要发布到crates.io 要求包名在其仓库上未被占用，抢注方式，先到先得！
license = "MulanPSL-2.0 OR Apache-2.0"     # 如果要发布到crates.io 要求声明采用的license，以告知用户可能在何种条款下使用该 crate
# 如果要发布到crates.io 要求对该 crate 进行描述，以告知用户该 crate 的作用
description = "A simple grep implementation from The Rust Programing Book.《Rust 程序设计语言》中的一个项目例程：获取一个文件名和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。"
version = "0.1.0"
authors = ["chry <a8512413@163.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

[Cargo 的文档](https://doc.rust-lang.org/cargo/) 描述了其他可以指定的元信息，他们可以帮助你的 crate 更容易被发现和使用！

### [发布到 Crates.io](https://rustwiki.org/zh-CN/book/ch14-02-publishing-to-crates-io.html#发布到-cratesio)

现在我们创建了一个账号，保存了 API token，为 crate 选择了一个名字，并指定了所需的元数据，你已经准备好发布了！发布 crate 会上传特定版本的 crate 到 [crates.io](https://crates.io/) 以供他人使用。

发布 crate 时请多加小心，因为发布是 **永久性的**（*permanent*）。对应版本不可能被覆盖，其代码也不可能被删除。[crates.io](https://crates.io/) 的一个主要目标是作为一个存储代码的永久文档服务器，这样所有依赖 [crates.io](https://crates.io/) 中的 crate 的项目都能一直正常工作。而允许删除版本没办法达成这个目标。然而，可以被发布的版本号却没有限制。

再次运行 `cargo publish` 命令。这次它应该会成功：

```text
$ cargo publish
 Updating registry `https://github.com/rust-lang/crates.io-index`
Packaging chry_minigrep v0.1.0 (file:///projects/chry_minigrep)
Verifying chry_minigrep v0.1.0 (file:///projects/chry_minigrep)
Compiling chry_minigrep v0.1.0
(file:///projects/chry_minigrep/target/package/chry_minigrep-0.1.0)
 Finished dev [unoptimized + debuginfo] target(s) in 0.19 secs
Uploading chry_minigrep v0.1.0 (file:///projects/chry_minigrep)
```

恭喜！你现在向 Rust 社区分享了代码，而且任何人都可以轻松的将你的 crate 加入他们项目的依赖。

### [发布现存 crate 的新版本](https://rustwiki.org/zh-CN/book/ch14-02-publishing-to-crates-io.html#发布现存-crate-的新版本)

当你修改了 crate 并准备好发布新版本时，改变 *Cargo.toml* 中 `version` 所指定的值。请使用 [语义化版本规则](http://semver.org/) 来根据修改的类型决定下一个版本号。接着运行 `cargo publish` 来上传新版本。

### [使用 `cargo yank` 从 Crates.io 撤回版本](https://rustwiki.org/zh-CN/book/ch14-02-publishing-to-crates-io.html#使用-cargo-yank-从-cratesio-撤回版本)

虽然你不能删除之前版本的 crate，但是可以阻止任何将来的项目将他们加入到依赖中。这在某个版本因为这样或那样的原因被破坏的情况很有用。对于这种情况，Cargo 支持 **撤回**（*yanking*）某个版本。

撤回某个版本会阻止新项目开始依赖此版本，不过所有现存此依赖的项目仍然能够下载和依赖这个版本。从本质上说，撤回意味着所有带有 *Cargo.lock* 的项目的依赖不会被破坏，同时任何新生成的 *Cargo.lock* 将不能使用被撤回的版本。

为了撤回一个 crate，运行 `cargo yank` 并指定希望撤回的版本：

```text
$ cargo yank --vers 0.1.0
```

也可以撤销撤回操作，并允许项目可以再次开始依赖某个版本，通过在命令上增加 `--undo`：

```text
$ cargo yank --vers 0.1.0 --undo
```

撤回 **并没有** 删除任何代码。举例来说，撤回功能并不意在删除不小心上传的秘密信息。如果出现了这种情况，请立即重新设置这些秘密信息。

## [使用 `cargo install` 从 Crates.io 安装二进制文件](https://rustwiki.org/zh-CN/book/ch14-04-installing-binaries.html#使用-cargo-install-从-cratesio-安装二进制文件)

`cargo install` 命令用于在本地安装和使用二进制 crate。它并不打算替换系统中的包；它意在作为一个方便 Rust 开发者们安装其他人已经在 [crates.io](https://crates.io/) 上共享的工具的手段。只有拥有二进制目标文件的包能够被安装。**二进制目标** 文件是在 crate 有 *src/main.rs* 或者其他指定为二进制文件时所创建的可执行程序，这不同于自身不能执行但适合包含在其他程序中的库目标文件。通常 crate 的 *README* 文件中有该 crate 是库、二进制目标还是两者都是的信息。

所有来自 `cargo install` 的二进制文件都安装到 Rust 安装根目录的 *bin* 文件夹中。如果你使用 *rustup.rs* 安装的 Rust 且没有自定义任何配置，这将是 `$HOME/.cargo/bin`。确保将这个目录添加到 `$PATH` 环境变量中就能够运行通过 `cargo install` 安装的程序了。

例如，第十二章提到的叫做 `ripgrep` 的用于搜索文件的 `grep` 的 Rust 实现。如果想要安装 `ripgrep`，可以运行如下：

```shell
chry@DESKTOP-UKSV006:/mnt/d/codes/git/studyrust/minigrep$ cargo install chry_minigrep
    Updating `git://mirrors.ustc.edu.cn/crates.io-index` index
  Downloaded chry_minigrep v0.1.0 (registry `git://mirrors.ustc.edu.cn/crates.io-index`)
  Downloaded 1 crate (7.0 KB) in 3.52s
  Installing chry_minigrep v0.1.0
   Compiling chry_minigrep v0.1.0
    Finished release [optimized] target(s) in 5.92s
  Installing /home/chry/.cargo/bin/chry_minigrep
   Installed package `chry_minigrep v0.1.0` (executable `chry_minigrep`)
chry@DESKTOP-UKSV006:/mnt/d/codes/git/studyrust/minigrep$
chry@DESKTOP-UKSV006:/mnt/d/codes/git/studyrust/minigrep$ chry_minigrep S Cargo.toml
运行参数： Args { inner: ["chry_minigrep", "S", "Cargo.toml"] }
license = "MulanPSL-2.0"     # 如果要发布到crates.io 要求声明采用的license，以告知用户可能在何种条款下使用该 crate
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
chry@DESKTOP-UKSV006:/mnt/d/codes/git/studyrust/minigrep$

```

最后一行输出展示了安装的二进制文件的位置和名称，在这里 `ripgrep` 被命名为 `rg`。只要你像上面提到的那样将安装目录加入 `$PATH`，就可以运行 `rg --help` 并开始使用一个更快更 Rust 的工具来搜索文件了！



## 使用 `cargo uninstall` 删除从 Crates.io 安装的二进制文件

```shell
chry@DESKTOP-UKSV006:/mnt/d/codes/git/studyrust/minigrep$ cargo uninstall chry_minigrep
    Removing /home/chry/.cargo/bin/chry_minigrep
chry@DESKTOP-UKSV006:/mnt/d/codes/git/studyrust/minigrep$

```

