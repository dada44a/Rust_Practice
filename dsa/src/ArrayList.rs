use crate::ListADT::{List, RandomAccessList, SequentialList};

pub struct ArrList<T> {
    data: Vec<T>,
}

impl<T> ArrList<T> {
    pub fn new() -> Self {
        ArrList { data: Vec::new() }
    }
}

impl<T> List for ArrList<T> {
    type Item = T;
    fn push(&mut self, value: T) {
        self.data.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn clear(&mut self) {
        self.data.clear()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T: Clone> RandomAccessList for ArrList<T> {
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    fn set(&mut self, index: usize, value: T) -> Option<T> {
        if let Some(x) = self.data.get_mut(index) {
            let a = x.clone();
            *x = value;
            Some(a)
        } else {
            None
        }
    }

    fn insert_at(&mut self, index: usize, value: T) {
        self.data.insert(index, value)
    }

    fn remove_at(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }
}
