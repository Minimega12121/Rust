use std::sync::Mutex;


fn main()  {
    let counter = Mutex::new(0);
    let mut handles = vec![];


    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();
            *count += 1;
        });

        handles.push(handle);
    }
}