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

fn main() {
    // thread_test1();
    thread_test2();
}
