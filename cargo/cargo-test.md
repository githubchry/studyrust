

# cargo test

## 参数

```
执行所有单元和集成测试，并建立本地软件包的实例。

使用方法。
    cargo test [OPTIONS] [TESTNAME] [-- <args>...] 。

OPTIONS:
    --q, --quiet 每个测试显示一个字符，而不是一行。
        --lib 只测试这个包的库的单元测试
        --bin <NAME>...              只测试指定的二进制文件
        --bins 测试所有二进制文件
        --example <NAME>...          只测试指定的例子
        --examples 测试所有的例子
        --测试<NAME>...             只测试指定的测试目标
        --tests 测试所有的测试
        --bench <NAME>...            只测试指定的工作台目标
        --benches 测试所有的benches
        --all-targets 测试所有目标
        --doc 只测试这个库的文档
        --no-run 编译，但不运行测试
        --no-fail-fast 运行所有测试，无论是否失败
    -p, --package <SPEC>...          要运行测试的软件包
        --all --工作区的别名(已废弃)
        --workspace 测试工作区的所有软件包
        --exclude <SPEC>...          从测试中排除软件包
    --j, --jobs <N> 并行作业的数量，默认为#个CPU
        --release 以发布模式构建工件，并进行优化
        --profile <PROFILE-NAME> 用指定的配置文件构建工件
        --features <FEATURES>...     空格或逗号分隔的要激活的特性列表
        --all-features 激活所有可用的特性
        --no-default-features 不激活 "default "功能
        --target <TRIPLE>...         为目标三元组构建
        --target-dir <DIRECTORY> 所有生成工件的目录
        --manifest-path <PATH> Cargo.toml的路径
        --ignore-rust-version 忽略软件包中的`rust-version`规范（不稳定的）。
        --message-format <FMT>...    错误格式
        --unit-graph 以JSON格式输出构建图（不稳定）。
        --future-incompat-report 在构建结束时输出一个未来的不兼容报告（不稳定）。
    -v, --verbose 使用verbose输出(-vv非常verbose/build.rs输出)
        --color <WHEN> 着色：自动、总是、从不
        --frozen 要求 Cargo.lock 和 cache 是最新的。
        --locked 要求Cargo.lock是最新的。
        --offline 在不访问网络的情况下运行
        --config <KEY=VALUE>...      覆盖一个配置值(不稳定)
    --Z <FLAG>...    给Cargo设置不稳定（只在夜间）的标志，详见 "cargo -Z help"。
    -h, --help 打印帮助信息

ARGS。
    <TESTNAME> 如果指定，只运行名称中包含这个字符串的测试
    <args>...     测试二进制的参数

通过www.DeepL.com/Translator（免费版）翻译


```



## 并行或连续的测试

当运行多个测试时， Rust 默认使用线程来并行运行。这意味着测试会更快地运行完毕，所以你可以更快的得到代码能否工作的反馈。因为测试是在同时运行的，你应该确保测试不能相互依赖，或依赖任何共享的状态，包括依赖共享的环境，比如当前工作目录或者环境变量。

举个例子，每一个测试都运行一些代码，假设这些代码都在硬盘上创建一个 *test-output.txt* 文件并写入一些数据。接着每一个测试都读取文件中的数据并断言这个文件包含特定的值，而这个值在每个测试中都是不同的。因为所有测试都是同时运行的，一个测试可能会在另一个测试读写文件过程中修改了文件。那么第二个测试就会失败，并不是因为代码不正确，而是因为测试并行运行时相互干扰。一个解决方案是使每一个测试读写不同的文件；另一个解决方案是一次运行一个测试。

如果你不希望测试并行运行，或者想要更加精确的控制线程的数量，可以传递 `--test-threads` 参数和希望使用线程的数量给测试二进制文件。例如：

```
$ cargo test -- --test-threads=1
```

这里将测试线程设置为 `1`，告诉程序不要使用任何并行机制。这也会比并行运行花费更多时间，不过在有共享的状态时，测试就不会潜在的相互干扰了。

## 显示函数输出

默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容。比如在测试中调用了 `println!` 而测试通过了，我们将不会在终端看到 `println!` 的输出：只会看到说明测试通过的提示行。如果测试失败了，则会看到所有标准输出和其他错误信息。

如果你希望也能看到通过的测试中打印的值，截获输出的行为可以通过 --nocapture 参数来禁用：

```
cargo test -- --nocapture
```



## 通过指定名字来运行部分测试

有时运行整个测试集会耗费很长时间。如果你负责特定位置的代码，你可能会希望只运行与这些代码相关的测试。
你可以向 cargo test 传递所希望运行的测试名称的参数来选择运行哪些测试。

```
cargo test this_test_will_fail
```

**需要注意的是如果测试名称存在包含关系时无法指定运行单个测试，因为任何名称匹配（包含）这个名称的测试会被运行。**

我们可以指定部分测试的名称，过滤运行多个测试。例如，因为头两个测试的名称包含 `works`，可以通过 `cargo test works` 来运行这两个测试：

```
cargo test works
```

同时注意测试所在的模块也是测试名称的一部分，所以可以通过模块名来运行一个模块中的所有测试。



有时一些特定的测试执行起来是非常耗费时间的，所以在大多数运行 `cargo test` 的时候希望能排除他们。虽然可以通过参数列举出所有希望运行的测试来做到，也可以使用 `ignore` 属性来标记耗时的测试并排除他们，如下所示：

```
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 需要运行一个小时的代码
}

```

 expensive_test 被列为 `ignored`，如果我们只希望运行被忽略的测试，可以使用 ` -- --ignored`参数：

```
cargo test -- --ignored
```



## 测试的组织结构

Rust 社区倾向于根据测试的两个主要分类来考虑问题：**单元测试**（*unit tests*）与 **集成测试**（*integration tests*）。单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口。而集成测试对于你的库来说则完全是外部的。它们与其他外部代码一样，通过相同的方式使用你的代码，只测试公有接口而且每个测试都有可能会测试多个模块。

为了保证你的库能够按照你的预期运行，从独立和整体的角度编写这两类测试都是非常重要的。

### 单元测试

单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确的某个单元的代码功能是否符合预期。单元测试与他们要测试的代码共同存放在位于 *src* 目录下相同的文件中。规范是在每个文件中创建包含测试函数的 `tests` 模块，并使用 `cfg(test)` 标注模块。

测试模块的 `#[cfg(test)]` 注解告诉 Rust 只在执行 `cargo test` 时才编译和运行测试代码，而在运行 `cargo build` 时不这么做。这在只希望构建库的时候可以节省编译时间，并且因为它们并没有包含测试，所以能减少编译产生的文件的大小。与之对应的集成测试因为位于另一个文件夹，所以它们并不需要 `#[cfg(test)]` 注解。然而单元测试位于与源码相同的文件中，所以你需要使用 `#[cfg(test)]` 来指定他们不应该被包含进编译结果中。

测试社区中一直存在关于是否应该对私有函数直接进行测试的论战，而在其他语言中想要测试私有函数是一件困难的，甚至是不可能的事。不过无论你坚持哪种测试意识形态，Rust 的私有性规则确实允许你测试私有函数。



### 集成测试

在 Rust 中，集成测试对于你需要测试的库来说完全是外部的。同其他使用库的代码一样使用库文件，也就是说它们只能调用一部分库中的公有 API 。集成测试的目的是测试库的多个部分能否一起正常工作。一些单独能正确运行的代码单元集成在一起也可能会出现问题，所以集成测试的覆盖率也是很重要的。

#### *tests* 目录

为了编写集成测试，需要在项目根目录创建一个 *tests* 目录，与 *src* 同级。Cargo 知道如何去寻找这个目录中的集成测试文件。接着可以随意在这个目录中创建任意多的测试文件，Cargo 会将每一个文件当作单独的 crate 来编译。

#### 集成测试中的子模块

随着集成测试的增加，你可能希望在 `tests` 目录增加更多文件以便更好的组织他们，例如根据测试的功能来将测试分组。正如我们之前提到的，每一个 *tests* 目录中的文件都被编译为单独的 crate。

将每个集成测试文件当作其自己的 crate 来对待，这更有助于创建单独的作用域，这种单独的作用域能提供更类似与最终使用者使用 crate 的环境。然而，正如你在第七章中学习的如何将代码分为模块和文件的知识，*tests* 目录中的文件不能像 *src* 中的文件那样共享相同的行为。

当你有一些在多个集成测试文件都会用到的帮助函数，而你尝试按照第七章 “将模块移动到其他文件” 部分的步骤将他们提取到一个通用的模块中时， *tests* 目录中不同文件的行为就会显得很明显。例如，如果我们可以创建 一个*tests/common.rs* 文件并创建一个名叫 `setup` 的函数，我们希望这个函数能被多个测试文件的测试函数调用：

文件名: tests/common.rs

```rust
pub fn setup() {
    // 编写特定库测试所需的代码
}
```

如果再次运行测试，将会在测试结果中看到一个新的对应 *common.rs* 文件的测试结果部分，即便这个文件并没有包含任何测试函数，也没有任何地方调用了 `setup` 函数：

```text
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b8b07b6f1be2db70

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-d993c68b431d39df

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

我们并不想要`common` 出现在测试结果中显示 `running 0 tests` 。我们只是希望其能被其他多个集成测试文件中调用罢了。

为了不让 `common` 出现在测试输出中，我们将创建 *tests/common/mod.rs* ，而不是创建 *tests/common.rs* 。这是一种 Rust 的命名规范，这样命名告诉 Rust 不要将 `common` 看作一个集成测试文件。将 `setup` 函数代码移动到 *tests/common/mod.rs* 并删除 *tests/common.rs* 文件之后，测试输出中将不会出现这一部分。*tests* 目录中的子目录不会被作为单独的 crate 编译或作为一个测试结果部分出现在测试输出中。

一旦拥有了 *tests/common/mod.rs*，就可以将其作为模块以便在任何集成测试文件中使用。这里是一个 *tests/integration_test.rs* 中调用 `setup` 函数的 `it_adds_two` 测试的例子：

文件名: tests/integration_test.rs

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

注意 `mod common;` 声明与示例 7-25 中展示的模块声明相同。接着在测试函数中就可以调用 `common::setup()` 了。

#### 二进制 crate 的集成测试

如果项目是二进制 crate 并且只包含 *src/main.rs* 而没有 *src/lib.rs*，这样就不可能在 *tests* 目录创建集成测试并使用 `extern crate` 导入 *src/main.rs* 中定义的函数。只有库 crate 才会向其他 crate 暴露了可供调用和使用的函数；二进制 crate 只意在单独运行。

为什么 Rust 二进制项目的结构明确采用 *src/main.rs* 调用 *src/lib.rs* 中的逻辑的方式？因为通过这种结构，集成测试 **就可以** 通过 `extern crate` 测试库 crate 中的主要功能了，而如果这些重要的功能没有问题的话，*src/main.rs* 中的少量代码也就会正常工作且不需要测试。

## 小结

Rust 的测试功能提供了一个确保即使你改变了函数的实现方式，也能继续以期望的方式运行的途径。单元测试独立地验证库的不同部分，也能够测试私有函数实现细节。集成测试则检查多个部分是否能结合起来正确地工作，并像其他外部代码那样测试库的公有 API。即使 Rust 的类型系统和所有权规则可以帮助避免一些 bug，不过测试对于减少代码中不符合期望行为的逻辑 bug 仍然是很重要的。