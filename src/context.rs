use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default, Clone)]
pub struct Context {
    inner: Rc<RefCell<Inner>>,
}

impl Context {
    pub fn get_node_id(&self) -> String {
        self.inner.borrow().node_id.clone()
    }

    pub fn next_id(&self) -> usize {
        let current = self.inner.borrow().next_id;
        self.inner.borrow_mut().next_id += 1;
        current
    }

    pub fn set_node_id(&self, node_id: impl Into<String>) {
        self.inner.borrow_mut().node_id = node_id.into();
    }

    pub fn set_node_ids(&self, node_ids: impl Into<Vec<String>>) {
        self.inner.borrow_mut().node_ids = node_ids.into();
    }
}

#[derive(Debug, Default)]
struct Inner {
    next_id: usize,
    node_id: String,
    node_ids: Vec<String>,
}
