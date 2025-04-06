use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

fn main() {
    println!("Hello, world!");
}

#[test]
fn threads_example() {
    let mut a = 1;

    let handle = thread::spawn(move || {
        a += 1;
        println!("{a}");
    });

    println!("Waiting for thread to finish...");
    println!("Thread finished...");
    handle.join().unwrap();
}

#[test]
fn multiple_producer_single_consumer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("11"),
            String::from("12"),
            String::from("13"),
            String::from("14"),
        ];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("21"),
            String::from("22"),
            String::from("23"),
            String::from("24"),
        ];

        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    for recieved in rx {
        println!("Got {recieved}")
    }
}

#[test]
fn rc_and_mutexes() {
    let num = 0;
    let counter = Arc::new(Mutex::new(num));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("After increment {}", *num);
        });  
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}

