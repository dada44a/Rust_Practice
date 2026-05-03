use crate::ListADT::{List, SequentialList};

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    cursor: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            cursor: 0,
        }
    }

    fn get_node_at_index_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        if index >= self.len() {
            return None;
        }

        let mut current = &mut self.head;
        for _ in 0..index {
            current = &mut current.as_mut()?.next;
        }
        current.as_mut().map(|boxed| &mut **boxed)
    }

    // ✅ FIXED: Uses &self.head, not &mut self.head
    fn get_node_at_index(&self, index: usize) -> Option<&Node<T>> {
        if index >= self.len() {
            return None;
        }
        let mut current = &self.head;
        for _ in 0..index {
            current = &current.as_ref()?.next;
        }
        current.as_ref().map(|boxed| &**boxed)
    }
}

impl<T> List for LinkedList<T> {
    type Item = T;

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            data: value,
            next: None,
        });

        let mut current = &mut self.head;

        while let Some(node) = current {
            current = &mut node.next;
        }

        *current = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                Some(node.data)
            }
            None => None,
        }
    }

    fn clear(&mut self) {
        self.head = None;
        self.cursor = 0;
    }

    fn len(&self) -> usize {
        let mut current = &self.head;
        let mut count: usize = 0;

        while let Some(node) = current {
            current = &node.next;
            count += 1;
        }
        count
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T: Clone> SequentialList for LinkedList<T> {

    // ✅ FIXED: cursor max is len-1
    fn move_next(&mut self) {
        if self.cursor < self.len() - 1 {
            self.cursor += 1;
        }
    }

    fn move_prev(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn insert_at_cursor(&mut self, value: T) {
        let len = self.len();

        if self.cursor == 0 {
            let new_node = Box::new(Node {
                data: value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        } else if self.cursor <= len {
            let prev_index = self.cursor - 1;

            if let Some(prev_node) = self.get_node_at_index_mut(prev_index) {
                let new_node = Box::new(Node {
                    data: value,
                    next: prev_node.next.take(),
                });
                prev_node.next = Some(new_node);
            }
        }
    }

    fn remove_at_cursor(&mut self) -> Option<T> {
        let len = self.len();
        if self.cursor >= len {
            return None;
        }

        let result = if self.cursor == 0 {
            self.head.take().map(|node| {
                self.head = node.next;
                node.data
            })
        } else {
            let prev_index = self.cursor - 1;
            if let Some(prev_node) = self.get_node_at_index_mut(prev_index) {
                prev_node.next.take().map(|node_to_remove| {
                    prev_node.next = node_to_remove.next;
                    node_to_remove.data
                })
            } else {
                None  // ✅ FIXED: Added missing None case
            }
        };

        // Adjust cursor after removal
        if result.is_some() {
            let new_len = self.len();
            if self.cursor >= new_len && new_len > 0 {
                self.cursor = new_len - 1;
            }
        }

        result
    }

    // ✅ ADDED: current() method
    fn current(&self) -> Option<&T> {
        self.get_node_at_index(self.cursor).map(|node| &node.data)
    }
}
