use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{mpsc, Arc, Mutex},
    thread::{self},
    time::Duration,
};

static mut CONTEXT: Vec<String> = Vec::new();
static mut TASKS: usize = 0;

fn main() {
    let mut destination = 0;
    let mut wait_timeout = 0;
    let mut wait_tasks = 0;

    unsafe {
        CONTEXT = Vec::with_capacity(43);
        for _i in 0..43 {
            CONTEXT.push(String::new());
        }
    }
    // 連接到ip 10.10.2.2 , 然後發送命令 "send cobra", 之後等待返回的消息, 連接timeout 1分鐘
    let pool = ThreadPool::new(4);
    for i in 1..=2 {
        destination += 1;
        pool.execute(move || {
            connect(i);
            println!("{i}.done");
        });
    }
    // println!("{}", context.lock().unwrap());

    unsafe {
        loop {
            if TASKS == destination || wait_timeout >= 60 {
                // unsafe {
                //     println!("{:?} ", CONTEXT);
                // }
                break;
            }

            thread::sleep(Duration::from_secs(1));
            if TASKS != wait_tasks {
                wait_tasks = TASKS;
                wait_timeout = 0;
                println!("TASKS: {TASKS} / {destination}");
            }
            wait_timeout += 1;
            println!("tick {wait_timeout}");
        }

        println!("{CONTEXT:?}\nTASKS: {TASKS} / {destination}");
    }
}
fn connect(id: usize) {
    println!("10.10.2.{}:6666", id);
    let mut stream = TcpStream::connect(format!("10.10.2.{}:6666", id)).unwrap();
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(30)))
        .unwrap();
    let mut buf = [0; 1024];
    stream.write_all(b"cobra").unwrap();
    loop {
        match stream.read(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let data = String::from_utf8_lossy(&buf[..bytes_read]);
                    let data = data.trim();
                    if !data.is_empty() && data != "processed msg" && data != "received msg" {
                        unsafe {
                            // CONTEXT.push_str(format!("\n10.10.2.{id}\n{data}\n").as_str());
                            CONTEXT[id - 1].push_str(data);
                            println!("{id} data: {}", data.len());
                        }
                    }
                    // waitting 60 sec to stop
                } else {
                    unsafe {
                        TASKS += 1;
                    }
                    //斷開連接
                    break;
                }
            }
            Err(e) => {
                unsafe {
                    TASKS += 1;
                }
                break;
            }
        }
    }
    // String::new()
}
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
enum Message {
    NewJob(Job),
    Terminate,
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // println!("Shutting down all workers.");

        for worker in &mut self.workers {
            // println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap_or(());
            }
        }
    }
}
#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    // println!("Worker {}; executing.", id);

                    job();
                }
                Message::Terminate => {
                    // println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
