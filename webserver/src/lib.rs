use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,   //thread pool contains a vec of workers, which are threads waiting for a task
    sender: mpsc::Sender<Message> //sender sends a Message
}

enum Message {  //Message is either a Job or Terminate
    NewJob(Job),
    Terminate
}

type Job = Box<dyn FnOnce() + Send + 'static>; //Job is a box of a type that implements FnOnce, Send, and 'Static
                                               //type just declares a synonym

impl ThreadPool {

    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The new function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); //panic if size param is not >0

        let (sender, receiver) = mpsc::channel(); //construct a channel - pass sender to thread pool - pass reveiver to worker
        let receiver = Arc::new(Mutex::new(receiver)); //since all threads will share a reveiver, we need a mutex to have concurrency

        let mut workers = Vec::with_capacity(size); //preallocate vector of size. more efficient than just adding to a vector

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self, f: F) //executre fnc with closure of type F as param
    where
        F: FnOnce() + Send + 'static, //the closure is only called once, the object can be sent to other objects, and the lifetime does not end at the end of scope
                                      //meaning that it can be passed between threads since it has its own lifetime
    {
        let job = Box::new(f); //construct a new job using the closure

        self.sender.send(Message::NewJob(job)).unwrap(); //send the job down the channel to a thread
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("sending terminate message to all workers");
        for worker in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");
        for worker in &mut self.workers {
            println!("shutting down worker {}", id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker { //a worker has an id and a thread
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop { //on thread::spawn you pass the closure, bascially the function the thread will run
            let message = receiver.lock().unwrap().recv().unwrap(); //lock the reciever so that we know only this thread is getting a job

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a new job", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was terminated", id);
                    break;
                }
            }
        }); 

        Worker {id, thread: Some(thread)}
    }
}