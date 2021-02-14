use super::*;

use std::fmt::Debug;

impl<T: Default + Debug + Eq> RingBuffer<T> {
    fn assert_push(&mut self, value: T) {
        assert_eq!(self.push(value), Ok(()));
    }
    fn assert_pop(&mut self, expected_value: T) {
        assert_eq!(self.pop(), Some(expected_value));
    }
}

#[test]
fn simple_push_pop() {
    let mut buf = RingBuffer::<i32>::with_capacity(10);
    buf.assert_push(1);
    buf.assert_push(2);
    buf.assert_pop(1);
    buf.assert_push(3);
    buf.assert_pop(2);
    buf.assert_pop(3);
}

#[test]
fn capacity_empty_full() {
    const SIZE: i32 = 100;
    let mut buf = RingBuffer::<i32>::with_capacity(SIZE as usize);

    assert!(buf.is_empty());
    assert!(!buf.is_full());
    assert_eq!(buf.pop(), None);

    for i in 0..SIZE {
        buf.assert_push(i);
    }

    assert!(!buf.is_empty());
    assert!(buf.is_full());
    assert_eq!(buf.push(777), Err(777));
    
    for i in 0..SIZE {
        buf.assert_pop(i);
    }
}

#[test]
fn multiple_cycles() {
    const SIZE: usize = 100;
    const BATCH_SIZE: usize = 77;
    const BATCH_CNT: usize = 25;
    let mut buf = RingBuffer::<i32>::with_capacity(SIZE);

    let mut write = (0..).into_iter();
    let mut read = (0..).into_iter();

    for _ in 0..BATCH_CNT {
        assert_eq!(buf.len(), 0);
        for _ in 0..BATCH_SIZE {
            buf.assert_push(write.next().unwrap());
        }
        assert_eq!(buf.len(), BATCH_SIZE);
        for _ in 0..BATCH_SIZE {
            buf.assert_pop(read.next().unwrap());
        }
        assert_eq!(buf.len(), 0);
    }
}
