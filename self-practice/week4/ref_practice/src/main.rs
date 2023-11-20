use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    data: i32,
    parent: RefCell<Option<Weak<Node>>>,
    next: RefCell<Option<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Node{
        data: 1,
        parent: RefCell::new(None),
        next: RefCell::new(None),
    });

    let b = Rc::new(Node{
        data: 1,
        parent: RefCell::new(Some(Rc::downgrade(&a))), // Rc -> Weak 不增加strong_count，只增加weak_count
        next: RefCell::new(None),
    });

    a.next.borrow_mut().replace(Rc::clone(&b));
    // drop(a);

    let c = b.parent.borrow().as_ref().unwrap().upgrade().unwrap();
    // as_ref: &Option<Weak<Node>> -> Option<&Weak<Node>>
    println!("{}", c.data);
    // println!("{}", Rc::strong_count(&a));
    // println!("{}", Rc::strong_count(&b));
}

