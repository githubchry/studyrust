
// ========================================================================
// 使用 use 关键字将名称引入作用域
// ========================================================================
// 在lib.rs，似乎我们编写的用于调用函数的路径都很冗长且重复，并不方便。
// 可以使用 use 关键字将路径一次性引入作用域，然后调用该路径中的项，就如同它们是本地项一样。
use restaurant::front_of_house::hosting;
// 在作用域中增加 use 和路径类似于在文件系统中创建软连接（符号连接，symbolic link）。
// 通过在 crate 根增加 use crate::front_of_house::hosting，现在 hosting 在作用域中就是有效的名称了，如同 hosting 模块被定义于 crate 根一样。
pub fn use_absolute_path() {
    hosting::add_to_waitlist();
}

// 通过 use 引入作用域的路径也会检查私有性，同其它路径一样。见lib.rs

// use甚至可以直接把最底层的函数引入作用域 但并不推荐这么做，原因：
// 在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化。
use restaurant::front_of_house::hosting::add_to_waitlist;
pub fn use_absolute_func() {
    add_to_waitlist();
    hosting::add_to_waitlist(); // 不会造成冲突和覆盖的情况
}

// ========================================================================
// 使用 as 关键字提供新的名称
// ========================================================================
//  Rust 不允许使用 use 语句将两个具有相同名称的项带入作用域
// 比如std::fmt和std::io都有Result类型，同时引入会造成冲突，Rust 则不知道我们要用的是哪个
// 除了使用全路径，还可以通过 as 重命名其中一个 Result 类型。
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}


fn main() {

    // restaurant::eat_breakfast_at_restaurant();


    // ========================================================================
    // 使用 pub use 重导出名称
    // ========================================================================
    /*
    当使用 use 关键字将名称导入作用域时，在新作用域中可用的名称是私有的。
    如果为了让调用你编写的代码的代码能够像在自己的作用域内引用这些类型，可以结合 pub 和 use。
    这个技术被称为 “重导出（re-exporting）”，因为这样做将项引入作用域并同时使其可供其他代码引入自己的作用域。

    见lib.rs里面的private_mod，没有pub声明，但是通过pub use 重导出名称后，可如下使用：
    */
    restaurant::re_export_mod::func();



    // ========================================================================
    // 使用外部包
    // ========================================================================
    /*
    Rust 标准库中尚未包含随机数功能，但提供了一个 rand crate
    在Cargo.toml的[dependencies]片段中加入rand依赖告诉Cargo要从crates.io下载rand和其依赖，并使其可在项目代码中使用。
    */
    use rand::Rng;
    /*
    Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中。
    rand::thread_rng 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。
    接下来，调用随机数生成器的 gen_range 方法。这个方法由刚才引入到作用域的 Rng trait 定义。
    gen_range 方法获取两个数字作为参数，并生成一个范围在两者之间的随机数。
    它包含下限但不包含上限，所以需要指定 1 和 101 来请求一个 1 和 100 之间的数。
    */

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("{}", secret_number);
}
