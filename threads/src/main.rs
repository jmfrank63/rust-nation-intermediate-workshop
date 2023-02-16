use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

struct ThreadPool {
    handles: Vec<thread::JoinHandle<()>>,
    sender: Sender<Box<dyn FnOnce() + Send + Sync + 'static>>,
}

impl ThreadPool
{
    fn new(number_of_threads: u8) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel::<Box<dyn FnOnce() + Send + Sync + 'static>>();
        let mut handles = vec![];
        for _ in 0..number_of_threads {
            handles.push(thread::spawn(|| {
                while let Ok(task) = receiver.recv() {
                    sender.send(task).unwrap();
                }
            }));
        }
        ThreadPool { handles, sender }
    }

    fn execute(&self, task: impl FnOnce() + Send + Sync + 'static) {
        self.sender.send(Box::new(task)).unwrap();
    }

    fn stop(&self) {
        drop(&self.sender);
        for handle in &self.handles {
            handle.join().unwrap();
        }
    }
}

fn main() {
    let pool = ThreadPool::new(10);

    pool.execute(|| {
        thread::sleep(Duration::from_secs(2));
        println!("SLOW Hello from thread");
    });

    for i in 0..15 {
        pool.execute(move || {
            println!("FAST Hello from thread for task: {}", i);
        });
    }

    // First we're making sure enough time is given to threads to execute the tasks
    // Then, replace this line with the `stop` method.
    thread::sleep(Duration::from_secs(3));
    pool.stop();
}
