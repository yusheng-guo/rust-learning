use std::cell::RefCell;
use std::rc::{Rc, self};

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

fn single_linked_list_create(vals:  Vec<u32>)->Option<Rc<RefCell<Node>>>{
    let list = Some(Rc::new(RefCell::new(Node::new(0, None))));

    for val in vals  {
        single_linked_list_insert(list.clone(),val);
    }

    list.unwrap().borrow().next.clone()
}

fn single_linked_list_insert(list:Option<Rc<RefCell<Node>>>, val: u32){
    let mut current = list.clone();
    let mut next = current.clone().map(|x| x.borrow().next.clone()).flatten();


    while next.is_some() && next.clone().map(|x| x.borrow().val).unwrap() <= val {
        current = next.clone();
        next = current.clone().map(|x| x.borrow().next.clone()).flatten();
    }

    match current.take() {
        Some(current_taken) =>{
            current_taken.borrow_mut().next = Some(Rc::new(RefCell::new(Node::new(val, next))));
            current = current_taken.borrow().next.clone();
        }
        None => {
            panic!("impossible")
        }
    };
}

fn main(){}