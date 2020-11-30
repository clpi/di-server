use std::{
    thread::{self, JoinHandle},
    io, sync::{mpsc, Arc, Mutex},
    rc,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::InvalidThreadNumber)
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Ok( Self { workers, sender } )
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("Shutting down worker {}.", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub enum PoolCreationError {
    InvalidThreadNumber,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let work_thread = thread::spawn(move || {
            while let Ok(msg) = receiver.lock().unwrap().recv() {
                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job, executing...", id);
                        job()
                    },
                    Message::Terminate => {
                        println!("Worker {} told to terminate", id);
                        break;
                    }
                }
            }
        });
        Self { id, thread: Some(work_thread) }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}
