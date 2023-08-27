
use std::{
    thread,
    sync::{mpsc, Arc, Mutex},

};


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce()  + Send + 'static>;
pub struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(_id: usize, receiver : Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", _id);
            job();
            
        });
        Worker { id:  _id, thread: Some(thread) }
    }
}

impl ThreadPool{
    pub fn new(_size: usize) -> ThreadPool {
        assert!(_size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(_size);
        for id in 0.._size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));

        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, _f:F) 
        where F: FnOnce() + Send + 'static,
        {
            let job = Box::new(_f);
            self.sender.send(job).unwrap();
        }

    // pub fn spawn<F, T>(f:F) -> JoinHandle<T>
    //     where 
    //         F: FnOnce() -> T,
    //         F: Send + 'static,
    //         T: Send + 'static,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
            //worker.thread.join().unwrap();
        }
    }
}
