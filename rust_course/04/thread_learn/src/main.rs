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

fn main() {
    // thread_test1();
    // thread_test2();
    // thread_test3();
    thread_test4();
}
