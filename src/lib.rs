#[cfg(test)]
mod tests;

pub struct RingBuffer<T: Default> {
    data: Vec<T>,
    begin: usize,
    end: usize
}

impl<T: Default> RingBuffer<T> {
    pub fn with_capacity(n: usize) -> RingBuffer<T> {
        let mut data = Vec::new();
        data.resize_with(n + 1, Default::default);
        RingBuffer {
            data,
            begin: 0,
            end: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.begin == self.end
    }

    pub fn is_full(&self) -> bool {
        self.next_pos(self.end) == self.begin
    }

    pub fn len(&self) -> usize {
        let mut end = self.end;
        if self.begin > self.end {
            end += self.data.len()
        }
        end - self.begin
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            Err(value)
        } else {
            self.data[self.end] = value;
            self.end = self.next_pos(self.end);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let result = Some(std::mem::take(&mut self.data[self.begin]));
            self.begin = self.next_pos(self.begin);
            result
        }
    }

    fn next_pos(&self, pos: usize) -> usize {
        (pos + 1) % self.data.len()
    }
}

impl<T: Default> IntoIterator for RingBuffer<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            ring_buffer: self
        }
    }
}

pub struct IntoIter<T: Default> {
    ring_buffer: RingBuffer<T>
}

impl<T: Default> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.ring_buffer.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.ring_buffer.len();
        (size, Some(size))
    }
}
