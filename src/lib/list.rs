use std::{cell::RefCell, fmt::Display, rc::Rc};

use super::node::{Node, NodeList};

pub struct List<T: Clone + Display> {
    head: Option<NodeList<T>>,
    size: usize,
    last: Option<NodeList<T>>,
}

impl<T: Clone + Display> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
            last: None,
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn get_head(&self) -> Option<&NodeList<T>> {
        self.head.as_ref()
    }
    pub fn get_last(&self) -> Option<&NodeList<T>> {
        self.last.as_ref()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn push_back(&mut self, item: T) {
        let node = Rc::new(RefCell::new(Node::new(item)));
        if let Some(last) = self.last.take() {
            last.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(Rc::clone(&last));
            self.last = Some(node);
            self.size += 1;
            return;
        }
        self.head = Some(Rc::clone(&node));
        self.last = Some(node);
        self.size = 1;
    }

    pub fn push_front(&mut self, item: T) {
        let node = Rc::new(RefCell::new(Node::new(item)));
        if let Some(prev_head) = self.head.take() {
            prev_head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(prev_head);
            self.head = Some(node);
            self.size += 1;
            return;
        }
        self.head = Some(Rc::clone(&node));
        self.last = Some(node);
        self.size = 1;
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.last.take().map(|last| {
            self.size -= 1;
            match last.borrow_mut().prev.take() {
                Some(node) => {
                    node.borrow_mut().next = None;
                    self.last = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            last.borrow().item.clone()
        })
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|prev_head| {
            self.size -= 1;
            match prev_head.borrow_mut().next.take() {
                Some(node) => {
                    node.borrow_mut().prev = None;
                    self.head = Some(node);
                }
                None => {
                    self.last.take();
                }
            }
            prev_head.borrow().item.clone()
        })
    }
    pub fn insert(&mut self, item: T, f: impl Fn(&T, &T) -> (bool, bool)) {
        if let Some(head) = &self.head {
            let mut current_node = Rc::clone(&head);
            let mut find_pos = false;
            let mut push_before = false;

            while let Some(next) = &Rc::clone(&current_node).borrow().next {
                let (exit, is_before) = f(&item, &current_node.borrow().item);
                push_before = is_before;
                if exit {
                    find_pos = true;
                    break;
                }

                current_node = Rc::clone(next);
            }

            // Last execution to last node!
            if !find_pos {
                let (_find, is_before) = f(&item, &current_node.as_ref().borrow().item);
                push_before = is_before;
            }
            self.append(&current_node, item, push_before);
            return;
        }

        // If not is head, created head and push;
        self.push_back(item);
    }
    fn append(&mut self, current_node: &NodeList<T>, item: T, push_before: bool) {
        let new_node = Rc::new(RefCell::new(Node::new(item.clone())));

        // Push before the current_node: `node`;
        if push_before {
            // let mut mut_current=current_node.borrow_mut()
            let exits_prev = current_node.borrow().prev.is_some();
            if exits_prev {
                let mut current_mut_node = current_node.borrow_mut();
                let prev = current_mut_node.prev.take().unwrap();
                current_mut_node.prev = Some(Rc::clone(&new_node));
                new_node.borrow_mut().next = Some(Rc::clone(current_node));
                prev.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(prev);
                self.size += 1;
            } else {
                self.push_front(item);
            }

            return;
        }
        // Else, push after the current_node:
        let exist_next = current_node.borrow().next.is_some();

        if exist_next {
            let mut current_mut_node = current_node.borrow_mut();
            let next = current_mut_node.next.take().unwrap();
            current_mut_node.next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(current_node));
            new_node.borrow_mut().next = Some(Rc::clone(&next));
            next.borrow_mut().prev = Some(new_node);
            self.size += 1;
        } else {
            self.push_back(item);
        }
    }
    pub fn delete_any(&mut self, mut f: impl FnMut(&Rc<RefCell<Node<T>>>) -> bool) {
        if let Some(head) = &self.head {
            let mut current_node = Rc::clone(head);
            let mut find_node = false;

            while let Some(next) = &Rc::clone(&current_node).borrow().next {
                if f(&current_node) {
                    find_node = true;
                    break;
                }
                current_node = Rc::clone(next);
            }

            // Missing test the last node: `current_node`
            if !find_node {
                if f(&current_node) {
                    find_node = true;
                }
            }

            //It is safe to see if the node has been found
            if !find_node {
                return;
            }

            if current_node.borrow().prev.is_none() {
                self.pop_front();
            } else if current_node.borrow().next.is_none() {
                self.pop_back();
            } else {
                // The current node is inside the list
                let mut current_mut = current_node.borrow_mut();
                let prev = current_mut.prev.take().unwrap();
                let next = current_mut.next.take().unwrap();
                prev.borrow_mut().next = Some(Rc::clone(&next));
                next.borrow_mut().prev = Some(prev);
                self.size -= 1;
            }
        }
    }
}

impl<T: Clone + Display> From<Vec<T>> for List<T> {
    fn from(items: Vec<T>) -> Self {
        let mut list = List::new();
        for item in items {
            list.push_back(item);
        }
        list
    }
}

impl<T: Clone + Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(head) = &self.head {
            let mut current_node = Rc::clone(&head);
            while let Some(next) = &Rc::clone(&current_node).borrow().next {
                write!(f, "({}) <-> ", current_node.borrow().item.clone())?;
                current_node = Rc::clone(&next);
            }
            write!(f, "({})", current_node.borrow().item.clone())?;
        } else {
            write!(f, "()")?;
        }
        return Ok(());
    }
}
