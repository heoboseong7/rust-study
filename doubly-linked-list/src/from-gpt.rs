#[derive(Clone, Debug)]
struct Node {
    value: i32,
    prev: Option<Box<Node>>,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            prev: None,
            next: None,
        }
    }
}

struct DoublyLinkedList {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn insert(&mut self, value: i32) {
        let mut new_node = Box::new(Node::new(value));
        if self.head.is_none() {
            self.head = Some(new_node);
            self.tail = self.head.clone();
        } else {
            let mut tail = self.tail.take().unwrap();
            let prevNext = tail.next;
            tail.next = Some(new_node);
            new_node.prev = prevNext;
            self.tail = Some(new_node);
        }
    }
}