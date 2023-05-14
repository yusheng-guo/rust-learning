use std::cell::RefCell;
use std::rc::Rc;

// struct Node{
//     int val,
//     struct Node *next,
// }


struct Node{
    val: u32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: u32, next:Option<Rc<RefCell<Node>>>) -> Self{
        Node { val, next}
    }
}