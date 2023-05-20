
struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32, next: Option<Box<Node>>) -> Node {
        Node {value: value, next: next}
    }
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList { head: None, size: 0 }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_enpty(&self) -> bool {
        self.get_size() == 0
    }

    pub fn push(&mut self, value: u32) {
        let new_node: Box<Node> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<u32>{
        let node: Box<Node> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }

    pub fn display(&self) {
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        println!("{}", result);
    }
}

fn main() {
    let mut list: LinkedList = LinkedList::new();
    println!("{}", list.is_enpty());
    for i in 1..10 {
        list.push(i);
    }
    list.display();
    println!("list size: {}", list.get_size());

    println!("top element: {}", list.pop().unwrap());
    list.display();
    println!("list size: {}", list.get_size());
}