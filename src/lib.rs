pub struct RingBuffer<T> {
    store: Vec<Option<T>>,
    start: usize,
    end: usize,
    buffer_len: usize,
}

impl<T> RingBuffer<T> {
    pub fn with_capacity(n: usize) -> RingBuffer<T> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(None);
        }
        RingBuffer { store: v, start: 0, end: 0, buffer_len: n }
    }
    pub fn push(&mut self, val: T) {
        if self.start == self.end {
            if let Some(_) = self.store[self.start] {
                self.start = (self.start + 1) % self.buffer_len;
            }
        }
        self.store[self.end] = Some(val);
        self.end = (self.end + 1) % self.buffer_len;
    }
    pub fn pop(&mut self) -> Option<T> {
        if let None = self.store[self.start] {
            None
        } else {
            let ret = std::mem::replace(&mut self.store[self.start], None);
            self.start = (self.start + 1) % self.buffer_len;
            ret
        }
    }
}

impl<T> IntoIterator for RingBuffer<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let RingBuffer { mut store, start, end, buffer_len } = self;
        store.rotate_left(start);
        if start <= end {
            store.truncate(end - start);
        } else {
            store.truncate(buffer_len - start + end);
        }
        store.reverse();
        IntoIter(store.into_iter().map(|x| x.unwrap()).collect())
    }
}

pub struct IntoIter<T>(Vec<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    use crate::RingBuffer;

    #[test]
    fn test_simple() {
        let mut buf = RingBuffer::with_capacity(3);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        buf.push(4);
        buf.pop();
        let v: Vec<_> = buf.into_iter().collect();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 3);
        assert_eq!(v[1], 4);
    }

    #[test]
    fn test_pop() {
        let mut buf = RingBuffer::with_capacity(3);
        buf.push(1);
        assert_eq!(buf.pop(), Some(1));
        assert_eq!(buf.pop(), None);
        assert_eq!(buf.pop(), None);
    }

    #[test]
    fn test_singleton() {
        let mut buf = RingBuffer::with_capacity(1);
        buf.push(1);
        buf.push(2);
        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.pop(), None);
        buf.push(3);
        assert_eq!(buf.pop(), Some(3));
    }
}
