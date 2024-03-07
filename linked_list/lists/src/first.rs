//2.4 Push
use std::mem; 

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    //2.2 New
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    //2.4 Push
    pub fn push(&mut self, elem: i32) {
        //let new_node = Node {
        //    elem: elem,
        //    next: self.head
        //};

        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty), //replace head temporarily
        });
        self.head = Link::More(new_node); //replace it back with added value
    }
    //2.5 Pop
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

//2.3 Ownership 101
//fn foo(self, arg2: Type)

//2.7 Drop
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

//2.6 Testing
#[cfg(test)] //only include this module when testing
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        //Check empty list behaves right
        assert_eq!(list.pop(), None);

        //populate list
        list.push(1);
        list.push(2);
        list.push(3);
        
        //check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        //Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);
        
        //check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        //check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}