pub mod queue;

use queue::Queue;
use queue::BlockingQueue;

fn main() {
    let mut q: BlockingQueue<i32> = BlockingQueue::new();

    let x = 4;

    for num in 0..x {
        q.push(num);
        print!("len = {}\n", q.len());
    }

    for _ in 0..x {
        let val = q.pop();
        print!("val = {}, len = {}\n", val.unwrap(), q.len());
    }
}