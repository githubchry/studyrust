/*
在餐饮业，餐馆中会有一些地方被称之为 前台（front of house），还有另外一些地方被称之为 后台（back of house）。
前台是招待顾客的地方，在这里，店主可以为顾客安排座位，服务员接受顾客下单，上菜和付款。
后台则是由厨师工作的厨房，洗碗工的工作地点，以及经理做行政工作的地方组成。


为了用Rust描述以上问题，可定义出前台front_of_house和后台back_of_house两大模块
    前台模块里面根据根据职能可划分出 店主(接待员)模块hosting 和 服务员模块serving
    后台模块里面根据根据职能可划分出 厨师模块cook 和 洗碗工模块dishwasher

定义一个模块：以 mod 关键字为起始，然后指定模块的名字，并且用花括号包围模块的主体。

下面模拟实现前台front_of_house模块：
*/
pub mod front_of_house {
    /*
    接待员模块 hosting

    在 mod hosting 前添加了 pub 关键字，使其变成公有的。
    伴随着这种变化，如果我们可以访问 front_of_house，那我们也可以访问 hosting。
    但是 hosting 的 内容（contents） 仍然是私有的；这表明使模块公有并不使其内容也是公有的。
    模块上的 pub 关键字只允许其父模块引用它。私有性规则不但应用于模块，还应用于结构体、枚举、函数和方法。
    综上，add_to_waitlist 也需要加上pub才能被外部引用
    */
    pub mod hosting {
        // 取号排队
        pub fn add_to_waitlist() {}

        // 安排座位
        fn seat_at_table() {}
    }

    // 服务员
    pub mod serving {
        // 下单
        fn take_order() {}

        // 上菜
        pub fn server_order() {}

        // 买单
        fn take_payment() {}
    }
}

/*
在前面我们提到了，src/main.rs 和 src/lib.rs 默认为 crate 根。
之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 crate 的模块，该结构被称为 模块树（module tree）。
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

这个树展示了一些模块是如何被嵌入到另一个模块的（例如，hosting 嵌套在 front_of_house 中）。
这个树还展示了一些模块是互为 兄弟（siblings） 的，这意味着它们定义在同一模块中（hosting 和 serving 被一起定义在 front_of_house 中）。
继续沿用家庭关系的比喻，如果一个模块 A 被包含在模块 B 中，我们将模块 A 称为模块 B 的 子（child），模块 B 则是模块 A 的 父（parent）。
注意，整个模块树都植根于名为 crate 的隐式模块下。

如何在模块树中找到一个项的位置？我们使用路径的方式，就像在文件系统使用路径一样。如果我们想要调用一个函数，我们需要知道它的路径。
路径有两种形式：
    绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
    相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。

在 crate 根定义了一个新函数 eat_at_restaurant，并在其中展示调用 add_to_waitlist 函数的两种方法
eat_at_restaurant 函数是我们 crate 库的一个公共API，所以我们使用 pub 关键字来标记它。
*/
pub fn eat_at_restaurant() {
    // 使用绝对路径方式调用
    crate::front_of_house::hosting::add_to_waitlist();

    // 使用相对路径方式调用: eat_at_restaurant 与 front_of_house 在同一级目录
    front_of_house::hosting::add_to_waitlist();
}
/*
选择使用相对路径还是绝对路径，取决于你是更倾向于将项的定义代码与使用该项的代码分开来移动，还是一起移动。

举一个例子，如果我们要将 front_of_house 模块和 eat_at_restaurant 函数一起移动到一个名为 customer_experience 的模块中，
我们需要更新 add_to_waitlist 的绝对路径，但是相对路径还是可用的。
然而，如果我们要将 eat_at_restaurant 函数单独移到一个名为 dining 的模块中，还是可以使用原本的绝对路径来调用 add_to_waitlist，
但是相对路径必须要更新。

我们更倾向于使用绝对路径，因为把代码定义和项调用各自独立地移动是更常见的。

我们还可以使用 super 开头来构建从父模块开始的相对路径。这么做类似于文件系统中以 .. 开头的语法。
我们为什么要这样做呢？下面模拟了厨师更正了一个错误订单，并将其提供给客户的情况。
*/
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::server_order();
    }

    pub fn cook_order() {}


    /*
    创建公有的结构体和枚举
    我们还可以使用 pub 来设计公有的结构体和枚举，不过有一些额外的细节需要注意。
    如果我们在一个结构体定义的前面使用了 pub ，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。
    我们可以根据情况决定每个字段是否公有。

    在一家餐馆中，顾客可以选择随餐附赠的面包类型，但是厨师会根据季节和库存情况来决定随餐搭配的水果。
    餐馆可用的水果变化是很快的，所以顾客不能选择水果，甚至无法看到他们将会得到什么水果。
    */
    pub struct Breakfast {
        pub toast: String,      // 面包
        seasonal_fruit: String, // 季节性水果
    }

    impl Breakfast {
        // 夏天的季节性水果为桃子
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //如果将枚举设为公有，则它的所有成员都将变为公有。
    //开胃菜
    pub enum Appetizer {
        Soup,   // 汤
        Salad,  // 沙拉
    }
}

pub fn eat_breakfast_at_restaurant() {
    // 在夏天点一份配有黑麦面包的早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");   // 编译错误: field `seasonal_fruit` of struct `Breakfast` is private

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// ========================================================================
// 使用 use 关键字将名称引入作用域
// ========================================================================
/*
目前为止，似乎我们编写的用于调用函数的路径都很冗长且重复，并不方便。
可以使用 use 关键字将路径一次性引入作用域，然后调用该路径中的项，就如同它们是本地项一样。
还可以使用 use 和相对路径来将一个项引入作用域。
use甚至可以直接把最底层的函数引入作用域 但并不推荐这么做

下面仅仅演示使用 use 和相对路径来将一个项引入作用域。其他方式见main.rs.
*/
use front_of_house::serving;
pub fn use_relative_path() {
    serving::server_order();
}

// ========================================================================
// 使用 pub use 重导出名称
// ========================================================================
/*
当使用 use 关键字将名称导入作用域时，在新作用域中可用的名称是私有的。
如果为了让调用你编写的代码的代码能够像在自己的作用域内引用这些类型，可以结合 pub 和 use。
这个技术被称为 “重导出（re-exporting）”，因为这样做将项引入作用域并同时使其可供其他代码引入自己的作用域。
*/
// private_mod没有pub关键字 一般情况下不能在外部使用（如main.rs）
mod private_mod {
    pub mod re_export_mod {
        pub fn func() {}
    }
}

pub use crate::private_mod::re_export_mod;
// 使用pub use重导出后就可以在外部（如main.rs）使用本地private模块


