#[derive(Debug)]
struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    fn enqueue(&mut self, val: T) -> Result<(), Sting> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        } 
        self.data.insert(0, val);

        Ok(())
    } 

    fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        0 == Self::size(&self)
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}