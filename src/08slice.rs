fn main() {
    // 假设一个需求：找到文本字符串中的第一个单词
    let mut s = String::from("hello world");
    println!("{:p} {:p} {} => s : {} ", &s, s.as_ptr(), s.len(), s);

    let word = first_word_length(&s); // word 的值为 5
    println!("word = {} ", word);

    // 此时得到的word就是文本字符串中第一个单词的长度，但是它是一个与 String 相分离的值，无法保证将来它仍然有效
    s.clear(); // 这清空了字符串，使其等于 ""

    //此时word就没用了，因为字符串已经变了，类似这样的代码逻辑bug在工程项目中会很常见
    //如何避免？——引入字符串 slice
    // ========================================================================
    // 字符串切片（string slice）
    // ========================================================================
    let mut s = String::from("hello world");

    // 使用 [starting_index..ending_index] 指定的 range 创建一个 slice，
    // 其中 starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值。
    // 注意：字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，编译器也会识别并提示编译错误
    // “字符串 slice” 的类型声明写作 &str
    // &s[..] 效果等同于 &s[0..s.len()]
    let hello = &s[0..5];   // 效果等同于 &s[..5]
    let blank = &s[5..6];
    let world = &s[6..11];  //&s[6..s.len()] 效果等同于 &s[6..]
    println!("{}{}{} ", hello, blank, world);

    //重写first_word_length函数为first_word，返回切片
    let word = first_word(&s); // word 的值为 "hello"
    println!("{:p} {:p} {} => s : {} ", &s, s.as_ptr(), s.len(), s);
    println!("{:p} {:p} {} => word : {} ", &word, word.as_ptr(), word.len(), word);

    s.clear(); // 这清空了字符串，使其等于 ""
    /*
    回忆一下借用规则，当拥有某值的不可变引用时，就不能再获取一个可变引用。
    因为 clear 需要清空 String，它尝试获取一个可变引用，而word是一个不可变引用&str。两者都是引用s
    Rust不允许这样做，因而编译失败，也在编译时就消除了一整类的错误！
    */
    // println!("{}", word);   // 编译错误：不能将 `s` 同时作为可变引用和不可变应用

    // ========================================================================
    // 经验之谈 使用切片作为函数入参
    // ========================================================================
    // first_word的入参是 String 引用，无法获取&str里面的第一个单词，可修改入参类型改进之
    let string = String::from("hello world");

    println!("{}", first_word_ex(&string[..]));  // 传入`String` 的 slice
    println!("{}", first_word_ex(&string));

    //-------------------------------------------------------------------------

    let str = "hello world";

    println!("{}", first_word_ex(&str[..]));
    println!("{}", first_word_ex(&str));
    println!("{}", first_word_ex(str)); //因为 `字符串字面值` 就是 `字符串 slice`，所以可不用slice语法

    // ========================================================================
    // 其他类型的 slice
    // ========================================================================
    let a = [10, 20, 30, 40, 50];
    let b = [10.0, 20.0, 30.0, 40.0, 50.0];

    println!("{:?}",  &a[1..3]);
    println!("{:?}",  &b[1..3]);
}

// 检索字符串第一个单词，返回单词长度
fn first_word_length(s: &String) -> usize {
    // 将 String 转化为字节数组
    let bytes = s.as_bytes();
    // 此时&bytes[0]实际上等于s.as_ptr()， 都是字符串在内存空间的起始地址
    println!("{:p} {:p} {} => s : {} ", &s, s.as_ptr(), s.len(), s);
    println!("{:p} {:p} {} => bytes : {:?} ", &bytes, &bytes[0], bytes.len(), bytes);

    /*
    使用 iter 方法在字节数组上创建一个迭代器：
    iter 方法返回集合中的每一个元素，而 enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回。
    enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用（注意是引用而不是拷贝）。
    使用`模式`来解构enumerate元组，因为元组第二个元素是集合中元素的引用特此做一点说明：
        (i, item)   => 引用本身就是一个地址值，这个地址值将绑定到item上，通过*item可以取出地址上存储的值
        (i, &item)  => &item表示一个引用值，item即表示引用的值，是没法打印其地址的(数值类没有实现`Pointer`)
    */
    if false {
        for (i, item) in bytes.iter().enumerate() {
            println!("{:p} {:p} {:?} => item  ", &item, item, item);
            //通过字节的字面值语法来寻找代表空格的字节
            if *item == b' ' {
                return i;
            }
        }
    } else {
        for (i, &item) in bytes.iter().enumerate() {
            println!("{:p} {:?} => &item  ", &item, item);
            //通过字节的字面值语法来寻找代表空格的字节
            if item == b' ' {
                return i;
            }
        }
    }

    s.len()
}

// 找到输入字符串中的第一个单词并返回
fn first_word(s: &String) -> &str { // “字符串 slice” 的类型声明写作 &str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]  // 效果等同于&s[0..s.len()]
}

// 找到输入字符串切片中的第一个单词并返回
fn first_word_ex(s: &str) -> &str { // “字符串 slice” 的类型声明写作 &str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]  // 效果等同于&s[0..s.len()]
}