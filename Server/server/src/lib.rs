
use std::{thread::{self}, sync::{mpsc, Arc, Mutex}};
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender:mpsc::Sender<Message>,

}
type Job = Box<dyn FnOnce() +Send+  'static >;

impl ThreadPool{
    /*
        Create new Threadpool

        size : number of threads in the pool
        
        #Panics:
            When size is 0
        
    */
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        let mut workers = Vec::with_capacity(size);
        let (sender,receiver) = mpsc::channel();
        //Receiver needs to be kept in thread safe smart pointers
        //Arc is for multiple thread safe ownership and for thread safe mutability we use mutex smart pointers
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        return ThreadPool{workers,sender};
    }
    pub fn execute<F>(&self,f:F)
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob((job))).unwrap();

         
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");
        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all Workers");
        for worker in &mut self.workers{
            println!("Shutting down worker {}",worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}
enum Message{
    NewJob(Job),
    Terminate,
}
struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}
impl Worker {
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Message>>>)->Worker{
        let thread = thread::spawn(move ||loop{
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job)=>{
                    println!("Worker {id} got a job;Executing");
                    job();
                }
                Message::Terminate=>{
                    println!("Worker {id} was told to terminate");
                    break;
                }
                
            }
        });
        return Worker{id, thread:Some((thread))};
    }
}


/*
    std::thread
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
*/