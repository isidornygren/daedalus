use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type WrappedCorridorNode = Rc<RefCell<CorridorNode>>;
type WeakWrappedCorridorNode = Weak<RefCell<CorridorNode>>;

pub struct CorridorNode {
    pub parent: Option<WeakWrappedCorridorNode>,
    pub children: Vec<WrappedCorridorNode>,
    pub x: u16,
    pub y: u16,
}

impl PartialEq for CorridorNode {
    fn eq(&self, other: &CorridorNode) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

pub fn set_parent(parent: &WrappedCorridorNode, child: &WrappedCorridorNode) {
    child.borrow_mut().parent = Some(Rc::downgrade(parent));
}

pub fn add_child(parent: &WrappedCorridorNode, child: &WrappedCorridorNode) {
    set_parent(parent, child);
    parent.borrow_mut().children.push(Rc::clone(child));
}

pub fn get_parent(node: &WrappedCorridorNode) -> Option<WrappedCorridorNode> {
    let parent = &node.borrow().parent;
    match parent {
        Some(p) => p.upgrade(),
        None => None,
    }
}

pub fn remove_node(node: &WrappedCorridorNode) {
    if let Some(parent) = get_parent(node) {
        parent
            .borrow_mut()
            .children
            .retain(|n| *n.borrow() != *node.borrow())
    }
}

impl CorridorNode {
    pub fn new(parent: Option<&WrappedCorridorNode>, x: u16, y: u16) -> WrappedCorridorNode {
        let node = Rc::new(RefCell::new(CorridorNode {
            parent: None,
            children: vec![],
            x,
            y,
        }));
        if let Some(p) = parent {
            add_child(&p, &node)
        }
        return node;
    }
}
