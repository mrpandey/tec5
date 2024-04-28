use std::sync::{Arc, Mutex};

use crate::queue::Queue;

type NodePtr<T> = Option<Arc<Mutex<Node<T>>>>;

struct Node<T> {
    value: T,
    next: NodePtr<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

pub struct BlockingQueue<T> {
    // number of elements in the queue
    length: usize,

    // head points at the front of the queue
    // we pop from front
    head: NodePtr<T>,
    
    // tail points at the back of the queue
    // we push at the back
    tail: NodePtr<T>,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }
}

impl<T> Queue<T> for BlockingQueue<T> {
    fn len(&self) -> usize {
        self.length
    }

    fn push(&mut self, t: T) {
        // we push at the tail

        let new_node = Arc::new(Mutex::new(Node::new(t)));
        
        match self.tail {
            None => {
                self.tail = Some(Arc::clone(&new_node));
                self.head = Some(Arc::clone(&new_node));
            },
            Some(ref mut node_rc) => {
                node_rc.lock().unwrap().next = Some(Arc::clone(&new_node));
                self.tail = Some(Arc::clone(&new_node));
            }
        }
        
        self.length += 1;
    }

    fn pop(&mut self) -> Option<T> {
        // we pop fron the head

        if self.length == 0 {
            return None;
        }

        if self.length == 1 {
            // head and tail both are strong references of this node
            // this will drop a strong reference so that we can call try_unwrap()
            self.tail = None
        }
        
        let result = Arc::try_unwrap(self.head.take().unwrap());
        let node = result.ok().unwrap().into_inner().unwrap();
        
        if self.length > 1 {
            self.head = Some(Arc::clone(node.next.as_ref().unwrap()));
        }
        
        self.length -= 1;
        
        return Some(node.value);
    }
}


#[cfg(test)]
mod tests {
    use crate::queue::Queue;
    use super::BlockingQueue;

    #[test]
    fn basic() {
        let mut q = BlockingQueue::<i32>::new();

        assert!(q.len() == 0);
        assert!(q.pop() == None);

        q.push(6);
        q.push(4);
        q.push(8);
        q.push(5);

        assert!(q.len() == 4);

        assert!(q.pop() == Some(6));
        assert!(q.pop() == Some(4));
        assert!(q.pop() == Some(8));

        assert!(q.len() == 1);

        // push again to check things don't get corrupted
        q.push(8);
        q.push(3);

        assert!(q.len() == 3);
        assert!(q.pop() == Some(5));
        assert!(q.pop() == Some(8));
        
        // exhaustion check
        assert!(q.pop() == Some(3));
        assert!(q.pop() == None);
        assert!(q.len() == 0);
    }
}