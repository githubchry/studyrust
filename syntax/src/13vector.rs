fn main() {
    // vector 允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值。
    // vector 是用泛型实现的, 只能储存相同类型的值。
    let v: Vec<i32> = Vec::new();   // new vector时必须指定类型注解，因为Rust 并不知道我们想要储存什么类型的元素。
    let v = vec![1, 2, 3];  // 新建一个包含初值的 vector，Rust 可以推断出 v 的类型是 Vec<i32>

    let mut v = Vec::new(); // 虽然没又指定类型注解，但是通过下文可推断出 v 的类型是 Vec<i32>
    v.push(5);  // 使用 push 方法向 vector 增加值
    v.push(6);
    v.push(7);
    v.push(8);
    print!("{:?}\n", v);

    // 丢弃 vector 时也会丢弃其所有元素
    {
        let v = vec![1, 2, 3, 4];
    }// <- 这里 v 离开作用域并被丢弃

    // 访问 vector 中一个值的两种方式，索引语法或者 get 方法
    let third: &i32 = &v[2];
    println!("v[2]：The third element is {}", third); // 打印7

    match v.get(2) {
        Some(third) => println!("v.get(2)： The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 推荐使用get方式访问 vector 因为：
    // 当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None。
    match v.get(10) {
        Some(third) => println!("get v[10] is {}", third),
        None => println!("There is no v[10]."),
    }
    // println!("v[10]： {}", v[10]); // 运行崩溃：数组越界index out of bounds

    /*
    一旦程序获取了一个有效的引用，借用检查器将会执行所有权和借用规则来确保 vector 内容的这个引用和任何其他引用保持有效。
    不能在相同作用域中同时存在可变和不可变引用的规则。
    */
    let first = &v[0];
    v.push(6);
    print!("{:?}\n", v);
    // println!("The first element is: {}", first);    // 编译错误：同时存在可变和不可变引用


    // 对应push，pop会移除并返回 vector 的最后一个元素
    let last = v.pop();
    match last {
        Some(val) => println!("vector last val is {}", val),
        None => println!("vector is null."),
    }
    print!("{:?}\n", v);


    // 遍历 vector 中的元素并修改：
    for i in &mut v {
        *i += 100;  // 先用*解引用，然后再更改
    }

    // 遍历 vector 中的元素
    for i in &v {
        println!("{}", i);
    }

    // vector作为参数  &[T] <=> Vec<T>
    let str_vec: Vec<&str> = vec!["hello", "world"];
    fn print_vec(args: &[&str]) {
        for s in args {
            println!("{}", s);
        }
    }
    print_vec(&str_vec);


    /*
    使用枚举来储存多种类型

    假如我们想要从电子表格的一行中获取值，而这一行的有些列包含数字，有些包含浮点值，还有些是字符串。
    我们可以定义一个枚举，其成员会存放这些不同类型的值，同时所有这些枚举成员都会被当作相同类型，那个枚举的类型。
    接着可以创建一个储存枚举值的 vector，这样最终就能够储存不同类型的值了。
    */
    #[derive(Debug)]    //增加注解来派生 Debug trait 以打印枚举内结构体数据
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    /*
    Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存。
    第二个好处是可以准确的知道这个 vector 中允许什么类型。
    如果 Rust 允许 vector 存放任意类型，那么当对 vector 元素执行操作时一个或多个类型的值就有可能会造成错误。
    使用枚举外加 match 意味着 Rust 能在编译时就保证总是会处理所有可能的情况

    如果在编写程序时不能确切无遗地知道运行时会储存进 vector 的所有类型，枚举技术就行不通了。
    (可以使用 trait 对象)
    */
}