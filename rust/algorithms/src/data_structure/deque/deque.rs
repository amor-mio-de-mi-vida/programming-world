#[derive(Debug)]

struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    fn new(size: usize) -> Self {
        Deque {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    fn add_front(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.push(val);
    
        Ok(())
    }

    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0,val);
        
        Ok(())
    }

    fn remove_front(&mut self) -> Option<T> {
        if Self::size(&Self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    fn remove_rear(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            Some(self.data.remove(0))
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