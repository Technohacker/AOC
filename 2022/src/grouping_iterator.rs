pub struct GroupingIterator<T, F: Fn(&T) -> bool, I: Iterator<Item = T>> {
    iter: I,
    cond: F,
    buf: Vec<T>,
}

impl<T, F: Fn(&T) -> bool, I: Iterator<Item = T>> GroupingIterator<T, F, I> {
    pub fn new(iter: I, cond: F) -> Self {
        Self {
            iter,
            cond,
            buf: vec![],
        }
    }
}

impl<T, F: Fn(&T) -> bool, I: Iterator<Item = T>> Iterator for GroupingIterator<T, F, I> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(x) => {
                    let split = (self.cond)(&x);
        
                    self.buf.push(x);
                    if split {
                        return Some(std::mem::take(&mut self.buf));
                    }
                },
                None => {
                    if self.buf.is_empty() {
                        return None;
                    }
                    return Some(std::mem::take(&mut self.buf));
                },
            }
        }
    }
}
