// 控制流 if - else

fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
        // "six"   // 编译错误 同组if-else必须返回同样类型
    };
    println!("The value of number is: {}", number);
    
    // while 条件循环
    let mut counter = 4;
    while counter != 0 {
        println!("{}!", counter);

        counter = counter - 1;
    }

    println!("LIFTOFF!!!");


    // loop 相当于 while true
    let result = loop {
        println!("loop!");
        counter += 1;

        if counter == number {
            break counter * 2;
        }
    };
    println!("The value of result is: {}", result);


    // for 循环遍历集合中的元素
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // for 循环来倒计时的例子 rev用来反转 range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
