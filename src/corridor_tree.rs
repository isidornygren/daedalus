use std::cell::RefCell;
use std::rc::Rc;

pub type WrappedCorridorNode = Rc<RefCell<CorridorNode>>;

pub struct CorridorNode {
    pub parent: Option<WrappedCorridorNode>,
    pub children: Vec<WrappedCorridorNode>,
    pub x: u16,
    pub y: u16,
}

impl PartialEq for CorridorNode {
    fn eq(&self, other: &CorridorNode) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

pub fn set_parent(node: &WrappedCorridorNode, parent: &WrappedCorridorNode) {
    node.borrow_mut().parent = Some(Rc::clone(parent));
}

pub fn add_child(node: &WrappedCorridorNode, child: &WrappedCorridorNode) {
    set_parent(node, child);
    node.borrow_mut().children.push(Rc::clone(child));
}

pub fn get_parent(node: &WrappedCorridorNode) -> Option<WrappedCorridorNode> {
    let parent = &node.borrow().parent;
    match parent {
        Some(p) => Some(Rc::clone(p)),
        None => None,
    }
}

pub fn remove_node(node: &WrappedCorridorNode) {
    let parent = get_parent(node);
    if parent.is_some() {
        parent
            .unwrap()
            .borrow_mut()
            .children
            .retain(|n| *n.borrow() != *node.borrow())
    }
}

impl CorridorNode {
    pub fn new(parent: Option<&WrappedCorridorNode>, x: u16, y: u16) -> WrappedCorridorNode {
        let node = Rc::new(RefCell::new(CorridorNode {
            parent: if parent.is_some() {
                Some(Rc::clone(parent.unwrap()))
            } else {
                None
            },
            children: vec![],
            x,
            y,
        }));
        if parent.is_some() {
            add_child(&parent.unwrap(), &node)
        }
        return node;
    }
}
