use std::io; // 引入std（标准库）里面的io（输入输出）库到当前作用域（命名空间）

// Rust 的缩进风格使用 4 个空格，而不是 1 个制表符（tab）。

fn main() {
    
    // ========================================================================
    // 不可变变量 可变变量 常量
    // ========================================================================
    // let定义一个变量variable(默认是不可改变的)，let加上mut声明该变量是可变的
    let immutable = "immutable"; // 不可变变量   类型：&str
    let mut mutable = "mutable"; // 可变变量   类型：&str
    println!("{} {}!", immutable, mutable);

    //immutable = "change immutable";   //编译错误，immutable是一个不可变变量，不能二次赋值
    mutable = "change mutable";
    println!("{}", mutable);

    //const定义一个常量，必须标注类型, 命名规范是使用下划线分隔的大写字母单词，并且可以在数字字面值中插入下划线来提升可读性
    const MAX_POINTS: u32 = 100_000;
    const KB: u32 = 1 << 10;
    const MINUTE: u16 = 60;
    const HOUR: u16 = MINUTE * 60;
    const PI_SYMBOL: char = 'π'; // 常量 : char
    const PI_VALUE: f64 = 3.1415926;
    println!("{} {} {} {} {}={}!", MAX_POINTS, KB, MINUTE, HOUR, PI_SYMBOL, PI_VALUE);

    //immutable = "change immutable";   //编译错误
    mutable = "change mutable";
    println!("{}", mutable);

    // ========================================================================
    // 变量隐藏：定义一个与之前变量同名的新变量，而新变量会隐藏之前的变量
    // ========================================================================
    let x = "test";
    let x = x.len() * 2; // 可改变类型
    println!("{}", x);
    let mut x = x; // 可变变量
    x += 8;
    // x = "test"; //编译错误：不能改变可变变量的类型
    println!("{}", x);

    // ========================================================================
    // 从键盘获取输入
    // ========================================================================
    println!("please input username:");
    // 使用String类型的关联函数new()创建一个空字符串 类型：std::string::String
    let mut username = String::new();
    /*
    如果前面没有声明use std::io;则需要std::io::stdin()获取终端标准输入句柄实例
    read_line从标准输入句柄获取一行用户输入, &表示这个参数是一个引用，引用默认是不可变的，故需增加mut声明
    read_line返回一个std::io::Result类型（枚举: Ok/Err），作用是编码错误处理信息，拥有 expect 方法。
    如果 std::io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect 的信息(可能是来源于底层操作系统错误的结果)。
    如果 std::io::Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。在本例中，这个值是用户输入到标准输入中的字节数。
    如果不调用 expect，程序也能编译，不过Rust会警告我们没有使用 read_line 的返回值 Result，说明有一个可能的错误没有处理。
    消除警告的正确做法是实际编写错误处理代码，不过由于我们就是希望程序在出现问题时立即崩溃，所以直接使用 expect。
    */
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    // 占位符{}
    println!("Hello, {}", username);
}
