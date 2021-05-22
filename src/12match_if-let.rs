fn main() {
    // ========================================================================
    // match 控制流运算符
    // ========================================================================
    /*
    `match`允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码
    `模式`可由字面值、变量、通配符和许多其他内容构成

    类似C/C++的switch，不过Rust能做更多

    下面来抽象一个例子：
    美国硬币有1、5、10、25美分四种面值，英文分别叫penny 、nickel 、dime、quarter
    1999 年到 2008 年间，美国在 25 美分的硬币的一侧为 50 个州的每一个都印刷了不同的设计。
    其他的硬币都没有这种区分州的设计，所以只有这些 25 美分硬币有特殊的价值。
    */
    #[derive(Debug)] // 这样可以可以立刻看到州的名称
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            // match分支有两个部分：`模式`和`代码`，两者由`=>`分开。
            // 分支代码较短的话通常不使用大括号，而使用逗号分隔分支
            Coin::Penny => {
                println!("Lucky penny!");
                1
            } // 分支代码使用大括号可不使用逗号分隔分支
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // 绑定值的模式
            Coin::Quarter(state) => {
                // 当匹配到 Coin::Quarter 时，变量 state 将会绑定 25 美分硬币所对应州的值。
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    let penny = value_in_cents(Coin::Penny);
    let quarter = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{} {} ", penny, quarter);

    // ========================================================================
    // 匹配是穷尽的
    // ========================================================================
    // C/C++的switch里面匹配不到的值会跑进default分支，不存在default分支则不进行任何操作。
    // 在Rust里面，使用下划线`_`通配符达到default分支的效果。并且我们必须显式保证match的穷尽性。
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // 如果注释掉则编译失败：必须保证match的穷尽性.. ()表示啥也不干
    }

    // 匹配 Option<T>
    // 我们想要编写一个函数，它获取一个 Option<i32> ，如果其中含有一个值，将其加一。
    // 如果其中没有值，函数应该返回 None 值，而不尝试执行任何操作。
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,   // 如果注释掉则编译失败：必须保证match的穷尽性
            Some(i) => Some(i + 1),
        }
    }

    // ========================================================================
    // if let 简单控制流
    // ========================================================================
    // 匹配一个 Option<u8> 值并只希望当值为 3 时执行代码：
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // 使用 if let 这种更短的方式编写
    if let Some(3) = some_u8_value {
        println!("three");
    }

    /*
    if let 获取通过等号分隔的一个模式和一个表达式。
    它的工作方式与 match 相同，这里的表达式对应 match 而模式则对应第一个分支。
    使用 if let 意味着编写更少代码，更少的缩进和更少的样板代码。然而，这样会失去 match 强制要求的穷尽性检查。
    match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。
    换句话说，可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

    当然，有if就可以有else:
    */
    if let Some(3) = some_u8_value {
        println!("three");
    } else if let Some(4) = some_u8_value {
        println!("four");
    } else {
        println!("not define");
    }
}