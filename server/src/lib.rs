use std::{thread::{self, Thread}, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send +'static>;

enum Message{
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool{
        assert!(size>0);
        
        let (sender,receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(
                id, 
                Arc::clone(&receiver)
            ));           
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static
    {
        //Channels are simple way to send info from one thread to another 

        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");

        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);

            //worker.thread.join().unwrap(); // worker is mut ref but join() method takes ownership. Instead of having worker store join handle, can have it hold option
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{ 
    id: usize,
    thread: Option<thread::JoinHandle<()>>  // Note function from signature of spawn, when we spawn a thread the return type is JoinHandle. 
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{ // Want workers to have shared ownership and mutablility of receiver. So we want to use thread safe smart pointers
        let thread = thread::spawn(move || loop{
            let message = receiver
                .lock() //Aquire a Mutex
                .unwrap()
                .recv() // recieve a job from the chanel
                .unwrap();
            //  if no job is available the thread that aquired the mutex to the reciever will wait for the job to be available.

            

            match message{
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing", id);
                    job();
                }
                Message::Terminate =>{
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            }
        });

        Worker{id, thread: Some(thread) }
    }
}