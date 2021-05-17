// Rust 不关心函数定义于何处，只要定义了就行，函数签名中必须声明每个参数的类型

fn add(x: i32, y: i32) -> i32 {
    println!("{} + {}", x, y);
    x + y // 返回值，结尾没有分号
}

fn main() {
    let n = add(4, 8);
    println!("{}", n);
    println!("{}", minus(4, 8));

    // 包含语句和表达式的函数体
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}", y);
}

fn minus(x: i32, y: i32) -> i32 {
    println!("{} - {}", x, y);
    x - y // 返回值，结尾没有分号
}
