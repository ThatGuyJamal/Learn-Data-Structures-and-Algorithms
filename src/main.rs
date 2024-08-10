use std::cell::RefCell;
use std::rc::Rc;

// Define the Node struct
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

// Define the LinkedList struct
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T>
where
    T: std::fmt::Debug,
{
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: self.tail.clone(),
        }));

        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(new_node.clone());
            }
            None => {
                self.head = Some(new_node.clone());
            }
        }

        self.tail = Some(new_node);
    }

    fn prepend(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.clone(),
            prev: None,
        }));

        match self.head.take() {
            Some(head) => {
                head.borrow_mut().prev = Some(new_node.clone());
            }
            None => {
                self.tail = Some(new_node.clone());
            }
        }

        self.head = Some(new_node);
    }

    fn print(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{:?} ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!();
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);
    list.prepend(0);

    list.print(); // Output: 0 1 2 3
}
