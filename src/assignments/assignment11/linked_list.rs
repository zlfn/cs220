//! Singly linked list.
//!
//! Consult <https://doc.rust-lang.org/book/ch15-01-box.html>.

use std::fmt::Debug;

/// Node of the list.
#[derive(Debug)]
pub struct Node<T: Debug> {
    /// Value of current node.
    pub value: T,

    /// Pointer to the next node. If it is `None`, there is no next node.
    pub next: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    /// Creates a new node.
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

/// A singly-linked list.
#[derive(Debug)]
pub struct SinglyLinkedList<T: Debug> {
    /// Head node of the list. If it is `None`, the list is empty.
    head: Option<Node<T>>,
}

impl<T: Debug> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> SinglyLinkedList<T> {
    /// Creates a new list.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Adds the given node to the front of the list.
    pub fn push_front(&mut self, value: T) {
        let old_head = self.head.take();
        self.head = Some(Node {
            value,
            next: old_head.map(Box::new)
        })
    }

    /// Adds the given node to the back of the list.
    pub fn push_back(&mut self, value: T) {
        let new_node = Node {
            value,
            next: None,
        };

        match &mut self.head {
            None => self.head = Some(new_node),
            Some(s) => {
                let mut x = &mut s.next;
                while let Some(n) = x {
                    x = &mut n.next;
                }
                *x = Some(Box::new(new_node))
            }
        }
    }

    /// Removes and returns the node at the front of the list.
    pub fn pop_front(&mut self) -> Option<T> {
        let Some(head) = self.head.take() else {
            return None
        };
        self.head = head.next.map(|x| *x);
        Some(head.value)
    }

    /// Removes and returns the node at the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        let mut current = self.head.as_mut()?;
        if current.next.is_none() {
            return self.head.take().map(|node| node.value);
        }

        // If current.next.next exist -> current = current.next
        while current.next.as_ref()?.next.is_some() {
            current = current.next.as_mut()?;
        }
        current.next.take().map(|node| node.value)
    }

    /// Create a new list from the given vector `vec`.
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut list = Self::new();
        for value in vec {
            list.push_back(value);
        }
        list
    }

    /// Convert the current list into a vector.
    pub fn into_vec(self) -> Vec<T> {
        let mut vec = Vec::new();
        match self.head {
            None => return vec,
            Some(s) => {
                vec.push(s.value);
                let mut x = s.next;
                while let Some(n) = x {
                    vec.push(n.value);
                    x = n.next;
                }
            }
        }
        vec
    }

    /// Return the length (i.e., number of nodes) of the list.
    pub fn length(&self) -> usize {
        let mut count = 0;
        match &self.head {
            None => return 0,
            Some(s) => {
                count += 1;
                let mut x = &s.next;
                while let Some(n) = x {
                    count += 1;
                    x = &n.next;
                }
            }
        }
        count
    }

    /// Apply function `f` on every element of the list.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2]`, `f`: `|x| x + 1` ==> `[2, 3]`
    pub fn map<F: Fn(T) -> T>(self, f: F) -> Self {
        let mut list = Self::new();
        match self.head {
            None => return list,
            Some(s) => {
                list.push_back(f(s.value));
                let mut x = s.next;
                while let Some(n) = x {
                    list.push_back(f(n.value));
                    x = n.next;
                }
            }
        }
        list
    }

    /// Apply given function `f` for each adjacent pair of elements in the list.
    /// If `self.length() < 2`, do nothing.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2, 3, 4]`, `f`: `|x, y| x + y`
    /// // each adjacent pair of elements: `(1, 2)`, `(2, 3)`, `(3, 4)`
    /// // apply `f` to each pair: `f(1, 2) == 3`, `f(2, 3) == 5`, `f(3, 4) == 7`
    /// ==> `[3, 5, 7]`
    pub fn pair_map<F: Fn(T, T) -> T>(self, f: F) -> Self
    where
        T: Clone,
    {
        let mut list = Self::new();
        let mut current = self.head;

        while current.as_ref().is_some_and(|s| s.next.is_some()) {
            let a = current.unwrap();
            let b = a.next.as_ref().unwrap();
            list.push_back(f(a.value.clone(), b.value.clone()));
            current = a.next.map(|x| *x);
        }
        list
    }
}

// A list of lists.
impl<T: Debug> SinglyLinkedList<SinglyLinkedList<T>> {
    /// Flatten the list of lists into a single list.
    ///
    /// # Examples
    /// `self`: `[[1, 2, 3], [4, 5, 6], [7, 8]]`
    /// ==> `[1, 2, 3, 4, 5, 6, 7, 8]`
    pub fn flatten(self) -> SinglyLinkedList<T> {
        let mut list = SinglyLinkedList::<T>::new();
        let mut current = self.head;
        while let Some(n) = current {
            let mut sub_list = n.value;
            let mut c = sub_list.head;
            while let Some(n) = c {
                list.push_back(n.value);
                c = n.next.map(|x| *x);
            }
            current = n.next.map(|x| *x);
        }
        list
    }
}
