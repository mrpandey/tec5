pub mod blocking_queue;
pub mod queue;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use queue::Queue;
use blocking_queue::BlockingQueue;

fn main() {
    let q: Arc<Mutex<BlockingQueue<i32>>> = Arc::new(Mutex::new(BlockingQueue::new()));

    let mut handles = vec![];

    let start_time = Instant::now();

    let num_threads = 1000;
    let num_ops = 1000;

    for i in 0..num_threads {
        let q_clone = Arc::clone(&q);

        let handle: thread::JoinHandle<()> = thread::spawn(move || {

            for j in 0..num_ops {
                q_clone.lock().unwrap().push(i*num_ops + j);
            }

            for _ in 0..num_ops {
                q_clone.lock().unwrap().pop();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start_time.elapsed();
    print!("Elapsed: {:.2?}\n", elapsed);
}
