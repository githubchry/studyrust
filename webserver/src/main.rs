use std::io::prelude::*;
// 获取读写流所需的特定 trait
use std::net::{TcpStream, TcpListener};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    // bind 函数返回 Result<T, E>，这表明绑定可能会失败，例如，连接 80 端口需要管理员权限（非管理员用户只能监听大于 1024 的端口），
    // 所以如果不是管理员尝试连接 80 端口，则会绑定失败。另一个例子是如果运行两个此程序的实例这样会有两个程序监听相同的端口，绑定会失败。
    // 因为我们是出于学习目的来编写一个基础的 server，将不用关心处理这类错误，使用 unwrap 在出现这些情况时直接停止程序。

    for stream in listener.incoming() {
        /*
        TcpListener 的 incoming 方法返回一个迭代器，它提供了一系列的流（更准确的说是 TcpStream 类型的流）。
        流（stream）代表一个客户端和服务端之间打开的连接。
        连接（connection）代表客户端连接服务端、服务端生成响应以及服务端关闭连接的全部请求 / 响应过程。
        为此，TcpStream 允许我们读取它来查看客户端发送了什么，并可以编写响应。
        总体来说，这个 for 循环会依次处理每个连接并产生一系列的流供我们处理。
        */
        let stream = stream.unwrap();  // 处理流的过程包含 unwrap 调用，如果出现任何错误会终止程序，如果没有任何错误，则打印出信息。

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // stream 参数是可变的。这是因为 TcpStream 实例在内部记录了所返回的数据。它可能读取了多于我们请求的数据并保存它们以备下一次请求数据。
    // 因此它需要是 mut 的因为其内部状态可能会改变；通常我们认为 “读取” 不需要可变性，不过在这个例子中则需要 mut 关键字。
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();  // 从 TcpStream 中读取字节并放入buffer中。

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    /*
    String::from_utf8_lossy 函数获取一个 &[u8] 并产生一个 String。
    函数名的 “lossy” 部分来源于当其遇到无效的 UTF-8 序列时的行为：它使用 �，U+FFFD REPLACEMENT CHARACTER，来代替无效序列。
    你可能会在缓冲区的剩余部分看到这些替代字符，因为他们没有被请求数据填满。
    */

    // 编写响应
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = std::fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    // 在 response 上调用 as_bytes，因为 stream 的 write 方法获取一个 &[u8] 并直接将这些字节发送给连接
    stream.write(response.as_bytes()).unwrap();
    // flush 会等待并阻塞程序执行直到所有字节都被写入连接中；TcpStream 包含一个内部缓冲区来最小化对底层操作系统的调用。
    stream.flush().unwrap();
}
