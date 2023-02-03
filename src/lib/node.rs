use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

pub type NodeList<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T: Display> {
    pub item: T,
    pub next: Option<NodeList<T>>,
    pub prev: Option<NodeList<T>>,
}

impl<T: Display> Node<T> {
    pub fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None,
        }
    }
}

impl<T: Display + Debug + Clone> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prev = self.prev.as_ref().map(|n| n.borrow().item.clone());
        let next = self.next.as_ref().map(|n| n.borrow().item.clone());
        write!(f, "({:?}) <- ({}) -> ({:?})", prev, self.item, next)?;
        Ok(())
    }
}
