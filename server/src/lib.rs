pub struct ThreadPool{
    workers: Vec<Worker>,
}
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id::usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker{ id , thread}
    }

}


impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// #Panics
    /// The 'new' function will panic if the size is zero
    
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            //create threads
        }

        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}