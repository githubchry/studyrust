/*
每一个编程语言都有高效处理重复概念的工具。在 Rust 中其工具之一就是 泛型（generics）。泛型是具体类型或其他属性的抽象替代。
我们可以表达泛型的属性，比如他们的行为或如何与其他泛型相关联，而不需要在编写和编译代码时知道他们在这里实际上代表什么。

假设需要设计一个函数，输入是一个数组，输出是这个数组的里面最大值
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

如果数组类型不是i32，而是i8 u8 i16 u16......那就凉凉，只能通过强转解决，如果是i32 u64，凉凉...
不可能每种类型都弄一个函数，因为函数体的代码是一样的，我们可以定义一个函数，再引进泛型参数来消除这种重复.

为了参数化新函数中的这些类型，我们也需要为类型参数取个名字，道理和给函数的形参起名一样。任何标识符都可以作为类型参数的名字。
这里选用 T，因为传统上来说，Rust 的参数名字都比较短，通常就只有一个字母，同时，Rust 类型名的命名规范是骆驼命名法（CamelCase）。
T 作为 “type” 的缩写是大部分 Rust 程序员的首选。

如果要在函数体中使用参数，就必须在函数签名中声明它的名字，好让编译器知道这个名字指代的是什么。
同理，当在函数签名中使用一个类型参数时，必须在使用它之前就声明它。
为了定义泛型版本的 largest 函数，类型参数声明位于函数名称与参数列表中间的尖括号 <> 中，像这样：

    fn largest<T>(list: &[T]) -> T {

可以这样理解这个定义：函数 largest 有泛型类型 T。它有个参数 list，其类型是元素为 T 的 slice。largest 函数的返回值类型也是 T。
*/

// 通过`+`指定PartialOrd和Copy两个trait bound，前者用于数值比较，后者用于数据拷贝，因为函数里面使用了这两功能
// 如果不加上这两个trait bound会导致编译失败...关于trait可详细看19trait.rs...这里主要把中心先放在泛型的概念上
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {

    // ========================================================================
    // 在函数定义中使用泛型
    // ========================================================================

    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);


    // ========================================================================
    // 在结构体中使用泛型
    // ========================================================================
    #[derive(Debug)]    //增加注解来派生 Debug trait 以打印枚举内结构体数据
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 }; //编译错误，x和y不是同一类型

    println!("integer{:?}  float{:?}", integer, float);

    // 泛型类型参数太多的话，代码将难以阅读和理解。当你的代码中需要许多泛型类型时，它可能表明你的代码需要重构，分解成更小的结构。
    #[derive(Debug)]    //增加注解来派生 Debug trait 以打印枚举内结构体数据
    struct PointEx<T, U> {
        x: T,
        y: U,
    }
    let both_integer = PointEx { x: 5, y: 10 };
    let both_float = PointEx { x: 1.0, y: 4.0 };
    let integer_and_float = PointEx { x: 5, y: 4.0 };
    println!("integer{:?}  float{:?} integer_and_float{:?}", both_integer, both_float, integer_and_float);


    // ========================================================================
    // 在枚举中使用泛型
    // ========================================================================
    /*
    回头看11enum.rs详看Option枚举
    enum Option<T> {
        Some(T),
        None,
    }

    回头看17result.rs详看Result枚举
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */

    // 当你意识到代码中定义了多个结构体或枚举，它们不一样的地方只是其中的值的类型的时候，不妨通过泛型类型来避免重复。


    // ========================================================================
    // 在方法中使用泛型
    // ========================================================================
    // 在前面定义的 Point<T> 结构体上实现方法 x，它返回 T 类型的字段 x 的引用
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    // 注意必须在 impl 后面声明 T，这样就可以在 Point<T> 上实现的方法中使用它了。
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // 在 impl 之后声明泛型 T ，这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型。
    // 例：为 Point<f32> 实例实现方法，而不是为泛型 Point 实例, 即该方法仅能用于f32类型的Point
    impl Point<f32> {
        // 返回Point距离原点的距离 powi计算平方 sqrt开方根 两者仅用于浮点型的数学运算符
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let float = Point { x: 1.0, y: 4.0 };
    println!("float.distance_from_origin = {}", float.distance_from_origin());

    /*
    结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型。
    为结构体 PointEx<T, U> 定义了一个方法 mixup。
    这个方法获取另一个 PointEx 作为参数，而它可能与调用 mixup 的 self 是不同的 PointEx 类型。
    */
    impl<T, U> PointEx<T, U> {
        fn mixup<V, W>(self, other: PointEx<V, W>) -> PointEx<T, W> {
            // 用 self 的 x 值（类型 T）和参数的 y 值（类型 W）来创建一个新 PointEx 类型的实例：
            PointEx {
                x: self.x,
                y: other.y,
            }
        }
    }

    // 这个例子的目的是展示一些泛型通过 impl 声明而另一些通过方法定义声明的情况。
    // 这里泛型参数 T 和 U 声明于 impl 之后，因为他们与结构体定义相对应。
    // 而泛型参数 V 和 W 声明于 fn mixup 之后，因为他们只是相对于方法本身的。

    let p1 = PointEx { x: 5, y: 10.4 };
    let p2 = PointEx { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    // ========================================================================
    // 泛型代码的性能
    // ========================================================================
    /*
    Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。
    Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
    单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

    分析一个使用标准库中 Option 枚举的例子：
    */
    let integer = Some(5);
    let float = Some(5.0);
    /*
    当 Rust 编译这些代码的时候，它会进行单态化。
    编译器会读取传递给 Option<T> 的值并发现有两种 Option<T>：一个对应 i32 另一个对应 f64。
    为此，它会将泛型定义 Option<T> 展开为 Option_i32 和 Option_f64，接着将泛型定义替换为这两个具体的定义。
    */
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);

    /*
    我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。
    这意味着在使用泛型时没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。
    这个单态化过程正是 Rust 泛型在运行时极其高效的原因。
    */
}