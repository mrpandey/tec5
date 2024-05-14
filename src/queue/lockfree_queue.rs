use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::ptr::{self, null};
use std::default::Default;

use crate::queue::Queue;

struct NodePtr<T: Default> {
    count: u32,
    ptr: AtomicPtr<Node<T>>,
}

type AtomicNodePtr<T: Default> = AtomicPtr<NodePtr<T>>;

struct Node<T: Default> {
    value: T,
    next: AtomicNodePtr<T>,
}

pub struct LockfreeQueue<T: Default> {
    // number of elements in the queue
    length: usize,

    // head points at the front of the queue
    // we pop from front
    head: AtomicNodePtr<T>,
    
    // tail points at the back of the queue
    // we push at the back
    tail: AtomicNodePtr<T>,
}

impl<T: Default> LockfreeQueue<T> {
    pub fn new() -> Self {
        let null_ptr : AtomicNodePtr<T> = AtomicNodePtr::default();

        let nd = Node {
            value: T::default(),
            next: null_ptr,
        };

        let mut nd_ptr = NodePtr {
            count: 0,
            ptr: AtomicPtr::new(Box::into_raw(Box::new(nd))),
        };

        Self {
            length: 0,
            head: AtomicPtr::new(&mut nd_ptr),
            tail: AtomicPtr::new(&mut nd_ptr),
        }
    }
}

impl<T: Default> Queue<T> for LockfreeQueue<T> {
    fn len(&self) -> usize {
        self.length
    }

    fn push(&mut self, t: T) {
        unimplemented!()
    }

    fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }
}