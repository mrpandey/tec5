use std::rc::Rc;
use std::cell::RefCell;

use crate::queue::Queue;

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

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

        let new_node = Rc::new(RefCell::new(Node::new(t)));
        
        match self.tail {
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(Rc::clone(&new_node));
            },
            Some(ref mut node_rc) => {
                node_rc.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
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
        
        let result = Rc::try_unwrap(self.head.take().unwrap());
        let node = result.ok().unwrap().into_inner();
        
        if self.length > 1 {
            self.head = Some(Rc::clone(node.next.as_ref().unwrap()));
        }
        
        self.length -= 1;
        
        return Some(node.value);
    }
}