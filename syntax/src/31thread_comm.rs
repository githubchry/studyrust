// mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写。
// Rust 标准库实现通道的方式意味着一个通道可以有多个产生值的 发送（sending）端，但只能有一个消费这些值的 接收（receiving）端。
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个通道，并将其两端赋值给 tx 和 rx, 即 发送者（transmitter）和 接收者（receiver）
    let (tx, rx) = mpsc::channel(); // mpsc::channel 函数返回一个元组：第一个元素是发送端，而第二个元素是接收端。

    // 在子线程中发送，在主线程接收
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
        // => 通道与所有权转移
        // println!("val is {}", val);  // 编译错误：val的所有权已经通过 tx.send 被转移给了tx通道
    });

    /*
    通道的接收端有两个有用的方法：recv 和 try_recv。

    recv方法会阻塞主线程执行直到从通道中接收一个值。一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。
    当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。

    try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。
    如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：
    可以编写一个循环来频繁调用 try_recv，在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。

    出于简单的考虑，这个例子使用了 recv；主线程中除了等待消息之外没有任何其他工作，所以阻塞主线程是合适的。
    */
    let received = rx.recv().unwrap();
    println!("received: {}", received);

    println!("通过克隆发送者来创建多个生产者");
    println!("试一下发送多个值并观察接收者的等待");

    let (tx, rx) = mpsc::channel(); // mpsc::channel 函数返回一个元组：第一个元素是发送端，而第二个元素是接收端。

    // 因为想创建多个发送者，就不能马上把tx的所有权转移出去，可按需clone几个出来
    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = mpsc::Sender::clone(&tx1);    // tx、tx1都是克隆，没啥区别
    // tx tx1 tx2 后面使用哪个都可以
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}