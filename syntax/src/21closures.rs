
use std::thread;
use std::time::Duration;

fn main() {
    /*
    Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。

    闭包不要求像 fn 函数那样在参数和返回值上注明类型。
    函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分。严格的定义这些接口对于保证所有人都认同函数使用和返回值的类型来说是很重要的。
    但是闭包并不用于这样暴露在外的接口：他们储存在变量中并被使用，不用命名他们或暴露给库的用户调用。

    闭包通常很短，并只关联于小范围的上下文而非任意情境。
    在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型，类似于它是如何能够推断大部分变量的类型一样。

    强制在这些小的匿名函数中注明类型是很恼人的，并且与编译器已知的信息存在大量的重复。
    类似于变量，如果相比严格的必要性你更希望增加明确性并变得更啰嗦，可以选择增加类型注解；

    有了类型注解闭包的语法就更类似函数了。如下是一个对其参数加一的函数的定义与拥有相同行为闭包语法的纵向对比。
    这里增加了一些空格来对齐相应部分。这展示了闭包语法如何类似于函数语法，除了使用竖线而不是括号以及几个可选的语法之外：
    */
    fn add_one_v1(x: u32) -> u32 { x + 1 }     //一个函数定义
    let add_one_v2 = |x: u32| -> u32 { x + 1 }; // 一个完整标注的闭包定义
    let add_one_v3 = |x| { x + 1 }; // 闭包定义中省略了类型注解 需要下文使用才能推断出来，否则编译错误
    let add_one_v4 = |x| x + 1; // 去掉了可选的大括号，因为闭包体只有一行 需要下文使用才能推断出来，否则编译错误
    println!("add_one_v3(10) = {}, add_one_v4(20) = {}", add_one_v3(10), add_one_v4(20));

    // 闭包定义会为每个参数和返回值推断一个具体类型。例如，示例 13-8 中展示了仅仅将参数作为返回值的简短的闭包定义。
    // 如果定义闭包并没有增加任何类型注解，尝试调用闭包两次，第一次使用 String 类型作为参数而第二次使用 u32，则会得到一个错误：
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // 编译错误 因为闭包的出入参数类型已在上一行被自动推断成String

    // ========================================================================
    // 闭包：可以捕获环境的匿名函数
    // ========================================================================
    // 上面简单演示只将闭包作为内联匿名函数来使用。
    // 不过闭包还有另一个函数所没有的功能：他们可以捕获其环境并访问其被定义的作用域的变量。
    let x = 4;
    let y = 4;
    let equal_to_x = |z| z == x;    // 引用了其周围作用域中变量x
    // 即便 x 并不是 equal_to_x 的一个参数，equal_to_x 闭包也被允许使用变量 x，因为它与 equal_to_x 定义于相同的作用域。
    // fn equal_to_x(z: i32) -> bool { z == x } // 编译错误：编译器甚至会提示我们这只能用于闭包！
    assert!(equal_to_x(y));

    /*
    当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。
    这会使用内存并产生额外的开销，在更一般的场景中，当我们不需要闭包来捕获环境时，我们不希望产生这些开销。
    因为函数从未允许捕获环境，定义和使用函数也就从不会有这些额外开销。

    闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。
    这三种捕获值的方式被编码为如下三个 Fn trait：
    1. FnOnce
        消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。
        为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。
        其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
    2. FnMut
        获取可变的借用值所以可以改变其环境
    3. Fn
        从其环境获取不可变的借用值

    当创建一个闭包时，Rust 根据其如何使用环境中变量来推断我们希望如何引用环境。
    由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce 。
    那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut 。
    而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn 。

    上面equal_to_x 闭包不可变的借用了 x（所以 equal_to_x 具有 Fn trait），因为闭包体只需要读取 x 的值。

    大部分需要指定一个 Fn 系列 trait bound 的时候，可以从 Fn 开始，而编译器会根据闭包体中的情况告诉你是否需要 FnMut 或 FnOnce。

    如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字。
    这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。
    */
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // 因为闭包使用 move 关键字定义，此时x已经被移动进了闭包，闭包获取了 x 的所有权
    //  main 就不再允许在 println! 语句中使用 x 了
    // println!("can't use x here: {:?}", x);   // 编译错误：main已经没有x的所有权了
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));


    /*
    考虑一下这个假定的场景：
    我们在一个通过 app 生成自定义健身计划的初创企业工作。
    其后端使用 Rust 编写，而生成健身计划的算法需要考虑很多不同的因素，
    比如用户的年龄、身体质量指数（Body Mass Index）、用户喜好、最近的健身活动和用户指定的强度系数。
    本例中实际的算法并不重要，重要的是这个计算只花费几秒钟。
    我们只希望在需要时调用算法，并且只希望调用一次，这样就不会让用户等得太久。
     */
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}

// ========================================================================
// 使用带有泛型和 Fn trait 的闭包
// ========================================================================
/*
创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。
你可能见过这种模式被称 memoization 或 lazy evaluation （惰性求值）。

为了让结构体存放闭包，我们需要指定闭包的类型，因为结构体定义需要知道其每一个字段的类型。
每一个闭包实例有其自己独有的匿名类型：也就是说，即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。
为了定义使用闭包的结构体、枚举或函数参数，需要使用泛型和 trait bound。

Fn 系列 trait 由标准库提供。所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个
注意：函数也都实现了这三个 Fn trait。如果不需要捕获环境中的值，则可以使用实现了 Fn trait 的函数而不是闭包。

这里使用 Fn trait。

为了满足 Fn trait bound 我们增加了代表闭包所必须的参数和返回值类型的类型。
在这个例子中，闭包有一个 u32 的参数并返回一个 u32，这样所指定的 trait bound 就是 Fn(u32) -> u32。
*/
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    // Cacher 结构体的字段是私有的，因为我们希望 Cacher 管理这些值而不是任由调用代码潜在的直接改变他们。
    calculation: T,
    // 存放闭包函数代码段
    value: Option<u32>, // 存放闭包函数返回值
}
/*
结构体 Cacher 有一个泛型 T 的字段 calculation。T 的 trait bound 指定了 T 是一个使用 Fn 的闭包。
任何我们希望储存到 Cacher 实例的 calculation 字段的闭包必须有一个 u32 参数（由 Fn 之后的括号的内容指定）并必须返回一个 u32（由 -> 之后的内容）。

想要实现的功能：
在执行闭包之前，value 将是 None。
如果使用 Cacher 的代码请求闭包的结果，这时会执行闭包并将结果储存在 value 字段的 Some 成员中。
接着如果代码再次请求闭包的结果，这时不再执行闭包，而是会返回存放在 Some 成员中的结果。

impl实现如下：
*/
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    // Cacher::new 返回一个在 calculation 字段中存放了指定闭包和在 value 字段中存放了 None 值的 Cacher 实例，因为我们还未执行闭包。
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,    // 在执行闭包之前，value 是 None
        }
    }

    // 如果使用 Cacher 的代码请求闭包的结果，这时会执行闭包并将结果储存在 value 字段的 Some 成员中。
    // 接着如果代码再次请求闭包的结果，这时不再执行闭包，而是会返回存放在 Some 成员中的结果。
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {

    // 不同于直接将闭包保存进一个变量，我们保存一个新的 Cacher 实例来存放闭包。
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // 接着，在每一个需要结果的地方，调用 Cacher 实例的 value 方法。
    // 可以调用 value 方法任意多次，或者一次也不调用，而慢计算最多只会运行一次。
    // 闭包打印的 calculating slowly... 只会在需要时出现并只会出现一次。

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}