/*
在 Rust 中，每一个值都属于某一个 数据类型（data type），这告诉 Rust 它被指定为何种数据，以便明确数据处理方式。
Rust 是 静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型。
根据值及其使用方式，编译器通常可以推断出我们想要用的类型。


数值运算： 加+ 减- 乘* 除/ 取余%

两类数据类型子集：标量（scalar）和复合（compound）。
    标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。
        整型
            无符号整型u8 u16 u32 u64 usize
            有符号整型i8 i16 i32(默认) i64 isize
        浮点型
            双精度浮点数f64(默认)   let x = 2.0;
            单精度浮点数f32     let y: f32 = 3.0;
        布尔型bool
            let t = true;
            let f: bool = false; // 显式指定类型注解
        字符类型
            字符''      let c = 'c';
            字符串""    let s = "string";
    复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。
        元组    let tup: (i32, f64, u8) = (500, 6.4, 1);

        数组
*/

fn main() {
    // 使用 parse 将 String 转换为数字时，必须增加类型注解，像这样：
    let integer: i32 = "42".parse().expect("Not a number!"); // 返回有可能是u8 f32 u32 i64等，所以必须指定类型注解
    println!("{}", integer);

    // ========================================================================
    // 整型字面值
    // ========================================================================
    let decimal = 98_222; //Decimal (十进制)
    let hex = 0xf; //Hex (十六进制)
    let octal = 0o77; //Octal (八进制)
    let binary = 0b1111_0000; //Binary (二进制)
    let byte = b'A'; // Byte (单字节字符)(仅限于u8)
    println!(
        "{} 0x{:02x} 0{:o} 0b{:b} {}",
        decimal, hex, octal, binary, byte
    );

    // ========================================================================
    // 整型溢出
    // ========================================================================
    let u8max: u8 = 255;
    println!("u8max = {}", u8max);
    let integer = u8max + 1; // 溢出：debug模式下崩溃 release模式下等于0
    println!("u8max + 1 = {}", integer);
    let integer = u8max + 2; // 溢出：debug模式下崩溃 release模式下等于1
    println!("u8max + 2 = {}", integer);

    // ========================================================================
    // 元组（tuple）
    // ========================================================================
    // 使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组长度固定：一旦声明，其长度不会增大或缩小。
    let tup = (500, 6.4, true, "hello"); // 创建了一个元组并绑定到 tup 变量上
                                         // let tup: (i32, f64, bool, &str) = (500, 6.4, true, "hello");   // 显式指定类型注解

    // 为了从元组中获取单个值, 使用模式匹配（pattern matching）来解构（destructure）元组值
    let (x, y, b, s) = tup;

    println!("{} {} {} {}", x, y, b, s);

    // ========================================================================
    // 数组（array）
    // ========================================================================
    let array = [1, 2, 3, 4, 5];
    // let array: [i32; 5] = [1, 2, 3, 4, 5];   // 显式指定类型注解
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{} {} {} {}", array[0], array[4], months[0], months[11]);

    let array = [3; 5]; // 创建size为5的数组，其上所有数初始化为3
    println!(
        "{} {} {} {} {}",
        array[0], array[1], array[2], array[3], array[4]
    );
}
