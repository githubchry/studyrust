use std::thread;
use std::time::Duration;

fn main() {
    // ========================================================================
    // 使用 spawn 创建新线程
    // ========================================================================
    // thread::spawn 的返回值类型是 JoinHandle。JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束。
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。
    handle.join().unwrap();

    // ========================================================================
    // 线程与 move 闭包
    // ========================================================================
    // 传递给 thread::spawn 的闭包并没有任何参数：并没有在新建线程代码中使用任何主线程的数据。
    // 为了在新建线程中使用来自于主线程的数据，需要新建线程的闭包获取它需要的值。
    let v = vec![1, 2, 3];

    // 通过在闭包之前增加 move 关键字，我们强制闭包获取其使用的值的所有权，而不是任由 Rust 推断它应该借用值。
    // 因为Rust 不知道这个新建线程会执行多久，所以无法知晓 v 的引用是否一直有效。万一在main销毁了v就会崩溃，所以编译器要求使用move
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // 编译错误：因为闭包使用了move，此时所有权已经转移

    handle.join().unwrap();
}

