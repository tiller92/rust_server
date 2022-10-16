use std::thread;



pub struct ThreadPool {
    threads:Vec<thread::JoinHandle<()>>,
}



impl ThreadPool {

    /// Creates a new thread
    /// the size is the number of threads in the pool.
    /// new panics if size is zero. obv not a good way to handle.
    ///
    ///
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); 

        let mut threads = Vec::with_capacity(size);

        ThreadPool {threads}
        }

      pub fn execute<F,T>(&self, f: F)
        where
            F:FnOnce()-> T,
            F:Send + 'static,
            T:Send + 'static,
      {
          }
}









