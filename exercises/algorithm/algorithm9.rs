/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + std::fmt::Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn print(&self, prefix: &str) {
        println!("{prefix} {{ Size: {}, Val: {:?}}}", self.count, self.items);
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.insert(self.count, value);

        let mut curr_idx = self.count;
        loop {
            let parent_idx = self.parent_idx(curr_idx);
            if parent_idx == 0 {
                break;
            }
            if (self.comparator)(&self.items[parent_idx], &self.items[curr_idx]) {
                break;
            }
            self.items.swap(parent_idx, curr_idx);
            curr_idx = parent_idx;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.count {
            0 => None,
            _ => {
                self.items.swap(0, 1);
                self.items.swap(1, self.count);
                self.count -= 1;
                // heapify
                let mut idx = 1;
                loop {
                    let child_idx = self.smallest_child_idx(idx);
                    if child_idx == idx {
                        break;
                    }
                    if !(self.comparator)(&self.items[idx], &self.items[child_idx]) {
                        self.items.swap(idx, child_idx);
                        idx = child_idx;
                    }
                    println!("curr: {}, child: {}", idx, child_idx);
                    break;
                }
                // ret val
                Some(std::mem::take(&mut self.items[0]))
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn node_present(&self, idx: usize) -> bool {
        idx <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // if not exist return idx
        let mut ret = idx;
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        if self.node_present(left_idx) {
            ret = left_idx
        }
        if self.node_present(right_idx)
            && !(self.comparator)(&self.items[left_idx], &self.items[right_idx])
        {
            ret = right_idx;
        }
        ret
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + std::fmt::Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
