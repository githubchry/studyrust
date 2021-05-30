
fn main() {

    /*
    HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。
    它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。
    HashMap是同质的：所有的键 K 必须是相同类型，值 V 也必须都是相同类型。

    需要使用 use 标准库中collections集合部分的 HashMap，因为 HashMap 没有被 prelude 自动引用
    */
    use std::collections::HashMap;

    // ========================================================================
    // 构建 HashMap 的两种方法
    // ========================================================================
    // 使用 new 创建一个空的 HashMap，并使用 insert 增加元素。
    let mut scores = HashMap::new();    //根据下文自动推导K的类型为String, V的类型为i32

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?} ", scores);

    /*
    使用一个元组 vector 的 collect 方法构建HashMap，其中每个元组包含一个键值对。
    collect 方法可以将数据收集进一系列的集合类型，包括 HashMap。
    如果队伍的名字和初始分数分别在两个 vector 中，可以使用 zip 方法来创建一个元组的 vector，其中 “Blue” 与 10 是一对，依此类推。
    接着就可以使用 collect 方法将这个元组 vector 转换成一个 HashMap
    */
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // 这里 HashMap<_, _> 类型注解是必要的，因为可能 collect 很多不同的数据结构，而除非显式指定否则 Rust 无从得知你需要的类型。
    // 但是对于键和值的类型参数来说，可以使用下划线占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型。
    // 值得注意的是，此时键类型K为&String，类型值类型V为&{integer}
    let field_name = String::from("Red");
    scores.insert(&field_name, &70);
    // scores.insert(String::from("Red"), 70);  //编译错误
    println!("{:?} ", scores);

    // ========================================================================
    // 哈希 map 和所有权
    // ========================================================================
    // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
    // 对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);    // 此后 field_name 和 field_value 不再有效，
    println!("{:?} ", map);
    // println!("{} {} ", field_name, field_value); // 编译错误：borrow of moved value


    // ========================================================================
    // 更新哈希 map
    // ========================================================================
    let mut scores = HashMap::new();
    // 覆盖插入
    scores.insert(String::from("Blue"), 15);
    scores.insert(String::from("Blue"), 25);
    println!("覆盖插入后：{:?}", scores);

    // 不存在key时插入 返回对应键K的V值的引用
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);

    // 根据旧值更新一个值
    let score = scores.entry(String::from("Blue")).or_insert(0);
    *score += 10;
    // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）。
    // 这里我们将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count。
    println!("根据旧值更新：{:?}", scores);


    // ========================================================================
    // 访问哈希 map 中的值
    // ========================================================================
    // 访问单个值
    let team_name = String::from("Red");
    let score = scores.get(&team_name);
    if let Some(val) = score {
        println!("Red => {}", val);
    } else {
        println!("Red does not exist!");
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(val) => println!("Blue => {}", val),
        None => println!("Blue does not exist!"),
    }

    // 遍历访问
    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }


    // ========================================================================
    // 切换哈希函数算法
    // ========================================================================
    /*
    HashMap 默认使用一种 “密码学安全的”（“cryptographically strong” ） 哈希函数
    它可以抵抗拒绝服务（Denial of Service, DoS）攻击。
    然而这并不是可用的最快的算法，不过为了更高的安全性值得付出一些性能的代价。
    如果性能监测显示此哈希函数非常慢，以致于你无法接受，你可以指定一个不同的 hasher 来切换为其它函数。
    hasher 是一个实现了 BuildHasher trait 的类型。
    你并不需要从头开始实现你自己的 hasher；crates.io 有其他人分享的实现了许多常用哈希算法的 hasher 的库。
    */

}