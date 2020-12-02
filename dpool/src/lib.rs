use crossbeam_channel::{Sender, Receiver, unbounded};
use std::{
    thread::{self, JoinHandle},
    io, sync::{mpsc, Arc, Mutex},
    rc,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    snd: Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: Option<usize>) -> Result<ThreadPool, PoolCreationError> {
        let num_workers = if let Some(num) = size {
            if num <= 0 { return Err(PoolCreationError::InvalidThreadNumber) }
            num
        } else { num_cpus::get() };
        let (snd, rcv) = unbounded();
        let rcv = Arc::new(Mutex::new(rcv));
        let mut workers = Vec::with_capacity(num_workers);
        for id in 0..num_workers {
            workers.push(Worker::new(id, Arc::clone(&rcv)));
        }
        Ok( Self { workers, snd } )
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.snd.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");
        for _ in &self.workers {
            self.snd.send(Message::Terminate).unwrap();
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
    fn new(id: usize, rcv: Arc<Mutex<Receiver<Message>>>) -> Self {
        let work_thread = thread::spawn(move || {
            while let Ok(msg) = rcv.lock().unwrap().recv() {
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
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
