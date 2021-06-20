// cargo run --bin 22iterator
// cargo test --bin 22iterator


// ========================================================================
// 使用迭代器处理元素序列
// ========================================================================
/*
迭代器模式允许你对一个序列的项进行某些处理。
迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。

在 Rust 中，迭代器是 惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果。
*/

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // 此时迭代器被储存在 v1_iter 变量中，而这时没有进行迭代。
    // 一旦 for 循环开始使用 v1_iter，接着迭代器中的每一个元素被用于循环的一次迭代，这会打印出其每一个值：
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // ========================================================================
    // Iterator trait 和 next 方法
    // ========================================================================
    /*
    迭代器的实现方式提供了对多种不同的序列使用相同逻辑的灵活性，而不仅仅是像 vector 这样可索引的数据结构.
    让我们看看迭代器是如何做到这些的。
    迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait。这个 trait 的定义看起来像这样：
        pub trait Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;

            // 此处省略了方法的默认实现
        }
    还未讲到的新语法：type Item 和 Self::Item，他们定义了 trait 的 关联类型（associated type）
    => 实现 Iterator trait 要求同时定义一个 Item 类型，这个 Item 类型被用作 next 方法的返回值类型。
        换句话说，Item 类型将是迭代器返回元素的类型。详情见36advanced_traits.rs

    next 是 Iterator 实现者被要求定义的唯一方法。next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None。
     */
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    /*
    注意 v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
    换句话说，代码 消费（consume）了，或使用了迭代器。每一个 next 调用都会从迭代器中消费一个项。
    使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。

    另外需要注意到从 next 调用中得到的值是 vector 的不可变引用。
    iter 方法生成一个不可变引用的迭代器。如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。
    类似的，如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter。
    */

    let mut v1 = v1;
    let v1_iter_mut = v1.iter_mut();
    for val in v1_iter_mut {
        *val *= 10;
    }
    println!("Got: {:?}", v1);

    let mut v1_into_iter = v1.into_iter();
    assert_eq!(v1_into_iter.next(), Some(10));
    assert_eq!(v1_into_iter.next(), Some(20));
    assert_eq!(v1_into_iter.next(), Some(30));
    assert_eq!(v1_into_iter.next(), None);

    // println!("Got: {}", v1);    // 编译错误：所有权被v1_into_iter获取了

    // ========================================================================
    // 消费迭代器的方法
    // ========================================================================
    /*
    Iterator trait 有一系列不同的由标准库提供默认实现的方法；你可以在 Iterator trait 的标准库 API 文档中找到所有这些方法。
    一些方法在其定义中调用了 next 方法，这也就是为什么在实现 Iterator trait 时要求实现 next 方法的原因。

    这些调用 next 方法的方法被称为 消费适配器（consuming adaptors），因为调用他们会消耗迭代器。
    一个消费适配器的例子是 sum 方法。这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。
    当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和。
     */
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // println!("{}", v1_iter.next());  // 编译错误：v1_iter的所有权已经再调用 sum 时被转移了

    // ========================================================================
    // 产生其他迭代器的方法
    // ========================================================================
    /*
    Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），他们允许我们将当前迭代器变为不同类型的迭代器。
    可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。

    展示了一个调用迭代器适配器方法 map 的例子，该 map 方法使用闭包来调用每个元素以生成新的迭代器。
    这里的闭包创建了一个新的迭代器，对其中 vector 中的每个元素都被加 1。
     */
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);   // 编译警告：所指定的闭包从未被调用过
    // 警告提醒了我们为什么：迭代器适配器是惰性的，而这里我们需要消费迭代器。
    // 为了修复这个警告并消费迭代器获取有用的结果，我们将使用collect 方法。这个方法消费迭代器并将结果收集到一个数据结构中。
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // 试一下把类型都换了
    let v3: Vec<_> = v2.iter().map(|x| x.to_string()).collect();
    assert_eq!(v3, vec!["2", "3", "4"]);

    // 因为 map 获取一个闭包，可以指定任何希望在遍历的每个元素上执行的操作。
    // 这是展示如何使用闭包来自定义行为同时又复用 Iterator trait 提供的迭代行为的绝佳例子。

}

// ========================================================================
// 使用闭包获取环境
// ========================================================================
/*
展示一个通过使用 filter 迭代器适配器和捕获环境的闭包的常规用例。
    cargo test --bin 22iterator

迭代器的 filter 方法获取一个使用迭代器的每一个项并返回布尔值的闭包。
如果闭包返回 true，其值将会包含在 filter 提供的新迭代器中。
如果闭包返回 false，其值不会包含在结果迭代器中。

*/
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// 获取一个鞋子 vector 的所有权和一个鞋子大小作为参数。它返回一个只包含指定大小鞋子的 vector。
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 使用 filter 和一个捕获环境中变量 shoe_size 的闭包，这样闭包就可以遍历一个 Shoe 结构体集合以便只返回指定大小的鞋子：
    shoes.into_iter()                       // 调用 into_iter 来创建一个获取 vector 所有权的迭代器
        .filter(|s| s.size == shoe_size)    // 将这个迭代器适配成一个只含有那些闭包返回 true 的元素的新迭代器
        .collect()                          // 将迭代器适配器返回的值收集进一个 vector 并返回。
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

// ========================================================================
// 实现 Iterator trait 来创建自定义迭代器
// ========================================================================
// Iterator trait 定义中唯一要求提供的方法就是 next 方法
// 演示自定义一个只会从 1 数到 5 的迭代器
// 首先，创建一个结构体来存放一些值
struct Counter {
    count: u32,     // 记录处理 1 到 5 的迭代过程中的位置。
    // count 是私有的因为我们希望 Counter 的实现来管理这个值。
}
// 接着实现 Iterator trait 将这个结构体放入迭代器中并在此实现中使用其值。
impl Counter {
    // new 函数通过总是从为 0 的 count 字段开始新实例来确保我们需要的行为。
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
// 接下来将为 Counter 类型实现 Iterator trait，通过定义 next 方法来指定使用迭代器时的行为，
impl Iterator for Counter {
    type Item = u32;    // 将迭代器的关联类型 Item 设置为 u32，意味着迭代器会返回 u32 值集合。

    fn next(&mut self) -> Option<Self::Item> {
        // 我们希望迭代器对其内部状态加一，这也就是为何将 count 初始化为 0：我们希望迭代器首先返回 1。
        self.count += 1;

        // 如果 count 值小于 6，next 会返回封装在 Some 中的当前值，不过如果 count 大于或等于 6，迭代器会返回 None。
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
// 测试使用 Counter 迭代器的 next 方法
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
/*
使用自定义迭代器中其他 Iterator trait 方法
通过定义 next 方法实现 Iterator trait，我们现在就可以使用任何标准库定义的拥有默认实现的 Iterator trait 方法了，因为他们都使用了 next 方法的功能。

例如，出于某种原因我们希望获取 Counter 实例产生的值，
将这些值与另一个 Counter 实例在省略了第一个值之后产生的值配对，
将每一对值相乘，只保留那些可以被三整除的结果，然后将所有保留的结果相加，这可以如示例中的测试这样做：
 */
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()       // 1 2 3 4 5
        .zip(Counter::new().skip(1))    // 1 2 3 4 5 + 2 3 4 5 => (1,2) (2,3) (3,4) (4,5)
        .map(|(a, b)| a * b)            // (1*2) (2*3) (3*4) (4*5) => 2 6 12 20
        .filter(|x| x % 3 == 0)         // 6 12
        .sum();                         // 18
    // 注意 zip 只产生四对值；理论上第五对值 (5, None) 从未被产生，因为 zip 在任一输入迭代器返回 None 时也返回 None。
    assert_eq!(18, sum);
}


// ========================================================================
// 性能对比：循环 VS 迭代器
// ========================================================================
/*
迭代器，作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能代码。
迭代器是 Rust 的 零成本抽象（zero-cost abstractions）之一，它意味着抽象并不会引入运行时开销.

Rust 对于已知长度的集合在编译时会进行“展开”（unroll）循环。展开是一种移除循环控制代码的开销并替换为每个迭代中的重复代码的优化。

所有的系数都被储存在了寄存器中，这意味着访问他们非常快。这里也没有运行时数组访问边界检查。
所有这些 Rust 能够提供的优化使得结果代码极为高效。
现在知道这些了，请放心大胆的使用迭代器和闭包吧！他们使得代码看起来更高级，但并不为此引入运行时性能损失。
*/