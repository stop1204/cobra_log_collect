use std::fmt::format;
use std::fs::File;
///SOM（System on Module）是一个嵌入式计算机板，而 M1 是其上运行的软件应用程序的一部分。在这些数据块中，我们可以看到有关 M1 状态的各种信息，例如传感器的读数、环境温度和压力、电流、设置温度等等。
// 下面是每个数据块中的一些值的解释：

// SW Version: 软件版本号
// CPU: SOM 上的 CPU 使用率
// SOM Memory Available: SOM 上可用的内存
// SD Free Space: 可用的 SD 卡空间
// SOM Storage: SOM 存储空间使用量
// SOM Time: SOM 时间戳
// M1 状态：

// T_CASE: M1 的外壳温度
// Set Temp: M1 设定温度
// RH: 相对湿度
// T-Discharge: 排气温度
// T-Liquid: 冷凝器温度
// T-Suction: 吸气温度
// T-Ambient: 环境温度
// P-Discharge: 排气压力
// P-Suction: 吸气压力
// Compression Ratio: 压缩比
// Compressor Amps: 压缩机电流
// Bypass Steps: 旁通阀步数
// SubCooling: 亚冷却度
// Superheat: 过热度
// ERRORS: 错误信息
// WARN: 警告信息
// M1 POWERBOX: M1 电源箱状态
// M1 FTC200 PV: FTC200 控制器压力值
// M1 FTC200 PWM: FTC200 控制器脉宽调制信号
///
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
    // test_connect();
    // return;
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
    let pool = ThreadPool::new(6);
    for i in 1..=43 {
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
            for i in 0..43 {
                    if !&CONTEXT[i].is_empty() {
                        let tmp = CONTEXT[i].clone();
                        &CONTEXT[i].clear();
                        output2(&tmp, i + 61);
                    }
                }

        }

        // println!("{CONTEXT:?}\nTASKS: {TASKS} / {destination}");
    }

    // output(unsafe { &CONTEXT });
}

/// 檢測是否連接到 沒連接到則創建bat文件
fn test_connect() {
        let mut output = String::new();
        for id in 1..=43{
           match TcpStream::connect(format!("10.10.2.{}:6666", id)){
                Ok(stream)=>(),
                Err(e)=>{
                    println!("CONNECT FAIL ID: {id}");
                    output.push_str(format!(r#"copy /y "\\10.10.2.{id}\3200\3200 ver\file-watch\file-watch.exe" "\\10.10.2.{id}\3200\3200 ver\file-watch\update""#).as_str());
                    output.push('\n');
                },
           }
        }
        // output to restart_filewatch.bat
        std::fs::write("restart_filewatch.bat", output);
    }
fn connect(id: usize) {
    println!("10.10.2.{}:6666", id);
    let mut stream = if let Ok(stream)= TcpStream::connect(format!("10.10.2.{}:6666", id)){
        stream
    }else{
        println!("ConnectionRefused");
        return;
    };
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
// 全部完成才輸出文件, 但是會發生中斷的情況
fn output(data: &Vec<String>) {
    let mut output = String::new();
    for i in 0..43 {
        if data[i].len() < 10 {
            // 跳過空內容
            continue;
        }

        output.push_str(format!("HSLTN {} ", i + 61).as_str());
        output.push_str("\n");
        output.push_str(cobra_log_collect::parse_handler(data[i].as_str()).as_str());
        output.push_str("\n");
    }

    // println!("{output}");

    // 将数据写入文件 output.csv
    let mut file = File::create("output.csv").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
// 完成一次就輸出一個文件
fn output2(data: &String, id: usize) {
    let mut output = String::new();

    if data.len() < 10 {
        // 跳過空內容
        return;
    }

    output.push_str(format!("HSLTN {} ", id + 61).as_str());
    output.push_str("\n");
    output.push_str(cobra_log_collect::parse_handler(data.as_str()).as_str());
    output.push_str("\n");

    // println!("{output}");

    // 将数据写入文件 output.csv
    let mut file = File::create(format!("output{id}.csv")).unwrap();
    file.write_all(output.as_bytes()).unwrap();
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
