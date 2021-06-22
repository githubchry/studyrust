use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,  // 通过通道来将Job发送给Worker去处理
}

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Vec::with_capacity 与 Vec::new 做了同样的工作，不过有一个重要的区别：它为 vector 预先分配空间。
        let mut workers = Vec::with_capacity(size);

        // 创建通道，每个 Worker 将会充当通道的接收端，ThreadPool作为发送端将Job发送给Worker
        let (sender, receiver) = mpsc::channel();
        /*
        Rust 所提供的通道实现是多生产者(sender-ThreadPool)，单消费者(receiver-Worker)的。
        而这里的业务场景是是一个生产者(sender-ThreadPool)多个消费者消费者(receiver-Worker)。
        我们希望通过在所有的 worker 中共享单一 receiver，在线程间分发任务。
        另外，从通道队列中取出任务涉及到修改 receiver，所以这些线程需要一个能安全的共享和修改 receiver 的方式，否则可能导致竞争状态
        为了在多个线程间共享所有权并允许线程修改其值，需要使用 Arc<Mutex<T>>。
        Arc 使得多个 worker 拥有接收端，而 Mutex 则确保一次只有一个 worker 能从接收端得到任务。
        */
        let receiver = Arc::new(Mutex::new(receiver));
        // 将通道的接收端放入一个 Arc 和一个 Mutex 中。对于每一个新 worker，克隆 Arc 来增加引用计数，如此这些 worker 就可以共享接收端的所有权了。
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    // 实现 execute 函数来获取传递的闭包并将其传递给池中的空闲线程执行
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        // FnOnce作为闭包trait： 处理请求的线程只会执行闭包一次，后面还会将闭包传给同样是FnOnce的spawn
        // FnOnce()代表一个没有参数也没有返回值的闭包
        // 需要 Send 来将闭包从一个线程转移到另一个线程(Worker)，而生命周期绑定'static是因为并不知道线程会执行多久
        let job = Box::new(f);  // => Box<dyn FnOnce() + Send + 'static> => type Job

        // 得到的闭包新建 Job 实例之后，将这些任务从通道的发送端发出
        self.sender.send(job).unwrap();
        /*
        这里调用 send 上的 unwrap，因为发送可能会失败，这可能发生于例如停止了所有线程执行的情况，这意味着接收端停止接收新消息了。
        不过目前我们无法停止线程执行；只要线程池存在他们就会一直执行。使用 unwrap 是因为我们知道失败不可能发生，即便编译器不这么认为。
        */
    }
}

// 使用type为设计的闭包类型创建别名: Job
type Job = Box<dyn FnOnce() + Send + 'static>;

/*
thread::spawn，它期望获取一些一旦创建线程就应该执行的代码。
然而，我们希望开始线程并使其等待稍后传递的代码。标准库的线程实现并没有包含这么做的方法；我们必须自己实现。

我们将要实现的行为是创建线程并稍后发送代码，这会在 ThreadPool 和线程间引入一个新数据类型来管理这种新行为。
这个数据结构称为 Worker：这是一个池实现中的常见概念。想象一下在餐馆厨房工作的员工：员工等待来自客户的订单，他们负责接受这些订单并完成它们。
*/
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                /*
                首先在 receiver 上调用了 lock 来获取互斥器，接着 unwrap 在出现任何错误时 panic。
                如果互斥器处于一种叫做 被污染（poisoned）的状态时获取锁可能会失败，这可能发生于其他线程在持有锁时 panic 了且没有释放锁。
                在这种情况下，调用 unwrap 使其 panic 是正确的行为。请随意将 unwrap 改为包含有意义错误信息的 expect。

                如果锁定了互斥器，接着调用 recv 从通道中接收 Job。最后的 unwrap 也绕过了一些错误，这可能发生于持有通道发送端的线程停止的情况，
                类似于如果接收端关闭时 send 方法如何返回 Err 一样。

                调用 recv 会阻塞当前线程，所以如果还没有任务，其会等待直到有可用的任务。Mutex<T> 确保一次只有一个 Worker 线程尝试请求任务。
                */
                println!("Worker {} got a job; executing.", id);

                job();  // 运行传递过来的闭包函数
            }

            /*
            这段代码可以编译和运行，但是并不会产生所期望的线程行为：一个慢请求仍然会导致其他请求等待执行。其原因有些微妙：
            Mutex 结构体没有公有 unlock 方法，因为锁的所有权依赖 lock 方法返回的 LockResult<MutexGuard<T>> 中 MutexGuard<T> 的生命周期。
            这允许借用检查器在编译时确保绝不会在没有持有锁的情况下访问由 Mutex 守护的资源，不过如果没有认真的思考 MutexGuard<T> 的生命周期的话，
            也可能会导致比预期更久的持有锁。因为 while 表达式中的值在整个块一直处于作用域中，job() 调用的过程中其仍然持有锁，这意味着其他 worker 不能接收任务。

            while let Ok(job) = receiver.lock().unwrap().recv() {

                println!("Worker {} got a job; executing.", id);

                job();
            }

            相反通过使用 loop 并在循环块之内而不是之外获取锁和任务，lock 方法返回的 MutexGuard 在 let job 语句结束之后立刻就被丢弃了。
            这确保了 recv 调用过程中持有锁，而在 job() 调用前锁就被释放了，这就允许并发处理多个请求了。
            */
        });

        Worker {
            id,
            thread,
        }
    }
}

