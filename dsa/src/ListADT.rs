pub trait List {
    type Item;
    fn clear(&mut self);
    fn push(&mut self, value: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait RandomAccessList: List {
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn set(&mut self, index: usize, value: Self::Item) -> Option<Self::Item>;
    fn insert_at(&mut self, index: usize, value: Self::Item);
    fn remove_at(&mut self, index: usize) -> Option<Self::Item>;
}

pub trait SequentialList: List {
    fn move_next(&mut self);
    fn move_prev(&mut self);
    fn insert_at_cursor(&mut self, value: Self::Item);
    fn remove_at_cursor(&mut self) -> Option<Self::Item>;
    fn current(&self) -> Option<&Self::Item>;
}
