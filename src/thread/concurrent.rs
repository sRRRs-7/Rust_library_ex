use std::{thread, time::Duration, sync::{mpsc, Mutex, Arc}};


pub fn main() {
    thread_func();
    thread_func2();
    thread_func3();
    thread_mutex();
}

// thread json
fn thread_func() {
    let arr = vec![1,2,3,4,5];
    let handle = thread::spawn(move || {
        for i in 1..=10 {
            println!("thread1 {}", i);
            println!("{:?}", arr);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..=10 {
        println!("thread2 {}", i);
        thread::sleep(Duration::from_millis(200));
    };

    handle.join().unwrap();
}


// channel
// mpsc -> multiple producer, single consumer -> sender : receiver = n : 1
fn thread_func2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("transmitter");
        tx.send(s).unwrap()
    });

    let recv = rx.recv().unwrap();
    println!("{}", recv)
}

// multi data send
fn thread_func3() {
    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let arr = vec![1, 2, 3, 4];
        for v in arr {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let arr = vec![11, 22, 33, 44];
        for v in arr {
            tx2.send(v).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for recv in rx {
        println!("{}", recv)
    }
}


// mutex -> mutual exclusion
// Arc -> atomic reference
fn thread_mutex() {
    let counter = Arc::new(Mutex::new(1));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += *num;
            println!("{}", num);
        });
        handles.push(handle);
    }

    for hand in handles {
        hand.join().unwrap()
    }

    println!("{:?}", counter.lock().unwrap());
}