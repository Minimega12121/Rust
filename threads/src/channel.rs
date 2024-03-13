use std::sync::mpsc; //multiple producer single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();
    //Thread 1 passing messages
    thread::spawn(move || {
        let vals = vec![
            String::from("hello world"),
            String::from("hello garv"),
            String::from("hello "),
            String::from("valo"),
        ];

        for msg in vals {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    //Thread 2 passing messages
    thread::spawn(move || {
        let vals = vec![
            String::from("hello world2"),
            String::from("hello garv2"),
            String::from("hello 2"),
            String::from("valo2"),
        ];

        for msg in vals {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for msg in rx {
        println!("{}", msg);
    }
}
