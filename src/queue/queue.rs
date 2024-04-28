pub trait Queue<T> {
    fn len(&self) -> usize;
    // fn front(&self) -> Option<&T>;
    fn push(&mut self, t: T);
    fn pop(&mut self) -> Option<T>;
}