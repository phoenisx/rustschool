use std::rc::Rc;
use std::cell::RefCell;

/** Box<T> example */
#[derive(Debug)]
struct BNode {
    id: usize,
    name: String,
    next: Box<Option<BNode>>,
}

impl BNode {
    fn new(id: usize, name: &str, next: Option<BNode>) -> Self {
        BNode{
            id,
            name: name.to_owned(),
            next: Box::new(next)
        }
    }
}
/** Box<T> example */

/** Rc<T> example */
#[derive(Debug)]
struct RList {
    head: Rc<RNodeOpt>,
}

#[derive(Debug)]
enum RNodeOpt {
    NODE(RNode),
    EMPTY,
}

#[derive(Debug)]
struct RNode {
    id: usize,
    next: Rc<RNodeOpt>,
}

impl RNode {
    fn new(id: usize, next: Rc<RNodeOpt>) -> Self {
        RNode{
            id,
            next,
        }
    }
}

fn main() {
    let tail: BNode = BNode::new(1, "Harry", None);
    let head: BNode = BNode::new(2, "Hermione", Some(tail));
    println!("Box<T> List:: {:?}", head.next);

    /*******************************************************
     * Rc<T>
     *******************************************************/

    // Following is the worst possible use-case of Rc<T>
    // We should avoid writing such code, as used in this example below...
    // We are Reference Counting both in RNode and RList, thus such a mess
    let rn2 = Rc::new(RNodeOpt::EMPTY);
    let rn1 = Rc::new(RNodeOpt::NODE(RNode::new(1, Rc::clone(&rn2))));
    let rn0 = Rc::new(RNodeOpt::NODE(RNode::new(0, Rc::clone(&rn1))));
    let r_list1 = RList {
        head: Rc::clone(&rn0),
    };
    let r_list2 = RList {
        head: Rc::clone(&rn1),
    };
    println!("Rc<T> List 1:: {:#?}", r_list1);
    println!("Rc<T> List 2:: {:#?}", r_list2);

    let v1 = Rc::new(RefCell::new(5));
    let v2 = Rc::clone(&v1);
    *v1.borrow_mut() += 10;
    *v2.borrow_mut() += 10;
    println!("Actual Value now = {:?}", v1);
}
