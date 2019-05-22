use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

use crate::thread_pool::message::Message;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = Some(thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        trace!("Worker {} got a job; executing.", id);
                        job.call_box();
                    }
                    Message::Terminate => {
                        trace!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        }));

        Worker {
            id,
            thread,
        }
    }
}
