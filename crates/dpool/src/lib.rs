// use crossbeam_channel::{Sender, Receiver, unbounded};
use std::{
    time::Duration,
    collections::HashMap,
    io, rc,
    sync::{
        mpsc::{Sender, Receiver, self},
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    snd: Sender<Msg>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: Option<usize>) -> Result<ThreadPool, PoolCreationError> {
        let num_workers = if let Some(num) = size {
            if num <= 0 { return Err(PoolCreationError::InvalidThreadNumber) }
            num
        } else { num_cpus::get() };
        // let (snd, rcv) = unbounded();
        let (snd, rcv): (Sender<Msg>, Receiver<Msg>) = mpsc::channel();
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
        self.snd.send(Msg::NewJob(job)).unwrap();
    }

    fn timer(send: mpsc::SyncSender<Msg>) -> io::Result<()> {
        let mut pulse = 0;
        loop {
            thread::sleep(Duration::from_secs(1));
            send.send(Msg::Pulse(pulse)).expect("Could not send pulse");
            pulse += 1;
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate Msg to all workers");
        for _ in &self.workers {
            self.snd.send(Msg::Terminate).unwrap();
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
    fn new(id: usize, rcv: Arc<Mutex<Receiver<Msg>>>) -> Self {
        let work_thread = thread::Builder::new()
            .name(format!("worker {}", id))
            .spawn(move || {
            while let Ok(msg) = rcv.lock().unwrap().recv() {
                match msg {
                    Msg::NewJob(job) => {
                        println!("Worker {} got a job, executing...", id);
                        job()
                    },
                    Msg::Terminate => {
                        println!("Worker {} told to terminate", id);
                        break;
                    }
                    _ => {  }
                }
            }
        }).expect("Could not create thread");
        Self { id, thread: Some(work_thread) }
    }
}

enum Msg {
    NewJob(Job),
    Terminate,
    Pulse(u16),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
