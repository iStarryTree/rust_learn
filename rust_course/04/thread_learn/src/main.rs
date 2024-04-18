use std::cell::RefCell;
use std::sync::{Arc, Barrier};
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
thread_local!{
    static FOO: RefCell<usize> = RefCell::new(0);
    
}

fn main() {
    // thread_test1();
    // thread_test2();
    // thread_test3();
    // thread_test4();
    // thread_test5();
    thread_test6();
}
