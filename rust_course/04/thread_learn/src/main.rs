use std::cell::RefCell;
use std::sync::{mpsc, Arc, Barrier};
use std::thread;
use std::time::Duration;

// 基本线程使用
#[allow(dead_code)]
fn thread_test1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawnd thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// 线程等待
#[allow(dead_code)]
fn thread_test2() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawnd thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// 线程的move
#[allow(dead_code)]
fn thread_test3() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // oh no!

    handle.join().unwrap();

    // println!("{:?}", v); // error: borrow of moved value: `v`
}

// 线程中没有堵塞的循环示范
#[allow(dead_code)]
fn thread_test4() {
    // 创建线程A
    let thread_a = thread::spawn(move || {
        // 创建线程B
        thread::spawn(move || loop {
            println!("I am threadB");
        });
    });

    thread_a.join().unwrap();
    println!("Child thread is finish!");

    thread::sleep(Duration::from_millis(100));
}

// 线程屏障
#[allow(dead_code)]
fn thread_test5() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// thread_local
#[allow(dead_code)]
fn thread_test6() {
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // 每个线程开始都拿到线程局部变量的初始值
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        })
    });

    t.join().unwrap();

    FOO.with(|f| assert_eq!(*f.borrow(), 2));
}
// 结构体中使用
thread_local! {
    static FOO: RefCell<usize> = RefCell::new(0);

}

// 消息通道
#[allow(dead_code)]
fn thread_test7() {
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("receive {}", rx.recv().unwrap());
}

// try_recv
#[allow(dead_code)]
fn thread_test8() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    print!("receive {}", rx.try_recv().unwrap()); // 没有消息会error
}

// 所有权
#[allow(dead_code)]
fn thread_test9() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("我，飞走了");
        tx.send(s).unwrap();
        // println!("send {}", s); // 没有所有权了
    });

    let received = rx.recv().unwrap();
    println!("receive {}", received);
}

// 使用for进行循环接收
#[allow(dead_code)]
fn thread_test10() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got: {}", received);
    }
}

// 使用多发送者
#[allow(dead_code)]
fn thread_test11() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // 不影响性能
    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from tx1")).unwrap();
    });

    for received in rx {
        println!("got: {}", received);
    }
}

// 异步通道
#[allow(dead_code)]
fn thread_test12() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(1));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

// 同步通道
#[allow(dead_code)]
fn thread_test13() {
    let (tx, rx) = mpsc::sync_channel(1);

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(1));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

// 传输多种类型的数据
#[allow(dead_code)]
enum Fruit {
    Apple(u8),
    Orange(String),
}
fn thread_test14() {
    let (tx, rx) = mpsc::channel();

    tx.send(Fruit::Apple(2)).unwrap();
    tx.send(Fruit::Orange(String::from("sweet"))).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(a) => println!("{} apples", a),
            Fruit::Orange(o) => println!("{} oranges", o),
        }
    }
}

fn main() {
    // thread_test1();
    // thread_test2();
    // thread_test3();
    // thread_test4();
    // thread_test5();
    // thread_test6();

    // 消息通道
    // thread_test7();
    // thread_test8();
    // thread_test9();
    // thread_test10();
    // thread_test11();
    // thread_test12();
    // thread_test13();
    thread_test14();
}
