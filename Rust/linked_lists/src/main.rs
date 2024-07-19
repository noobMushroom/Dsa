use std::{cell::RefCell, rc::Rc};

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}

#[derive(Clone)]
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

impl TransactionLog {
    fn new_empty() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    fn get_node_at_index(&self, index: u64) -> Option<Node> {
        if self.length < index {
            return None;
        }

        let mut curr_index = 0;

        let mut current = self.head.clone();
        while let Some(value) = current {
            if curr_index == index {
                return Some(value.borrow().clone());
            }

            current = value.borrow().next.clone();
            curr_index += 1;
        }

        None
    }

    fn find_node_index(&self, value: String) -> Option<u32> {
        let mut current = self.head.clone();
        let mut index = 0;

        while let Some(node) = current {
            if node.borrow().value == value {
                return Some(index);
            }
            current = node.borrow().next.clone();
            index += 1;
        }
        None
    }

    fn append(&mut self, value: String) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => old_tail.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }

        self.tail = Some(new_node);
        self.length += 1;
    }

    fn print_list(&self) {
        let mut current_node = self.head.clone();

        while let Some(node) = current_node {
            let node_ref = node.borrow();
            println!("{}", node_ref.value);
            current_node = node_ref.next.clone();
        }
    }

    fn get_vec(&self) -> Vec<String> {
        let mut current_node = self.head.clone();
        let mut node_values_vec = vec![];

        while let Some(node) = current_node {
            let node_ref = node.borrow();
            node_values_vec.push(node_ref.clone().value);
            current_node = node_ref.next.clone();
        }

        node_values_vec
    }

    fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.length = 0;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;
    fn get_list() -> TransactionLog {
        let mut new_list = TransactionLog::new_empty();
        new_list.append("cool".to_string());
        new_list.append("cooler".to_string());
        new_list.append("somethign".to_string());
        new_list.append("fool".to_string());
        new_list.append("tool".to_string());
        new_list
    }

    #[test]
    fn check_vec() {
        let new_list = get_list();
        assert_eq!(
            vec![
                "cool".to_string(),
                "cooler".to_string(),
                "somethign".to_string(),
                "fool".to_string(),
                "tool".to_string()
            ],
            new_list.get_vec()
        );
    }

    #[test]
    fn clear_list() {
        let mut new_list = get_list();
        new_list.clear();
        let expected_vec: Vec<String> = vec![];
        assert_eq!(expected_vec, new_list.get_vec());
    }

    #[test]
    fn find_node_index() {
        let new_list = get_list();
        assert_eq!(new_list.find_node_index("cooler".to_string()), Some(1));
    }

    #[test]
    fn get_value_at_any_index() {
        let new_list = get_list();
        let test_node = new_list.get_node_at_index(1);
        assert!(test_node.is_some()); // Check if the node exists
        if let Some(node) = test_node {
            let node_ref = node;
            assert_eq!(node_ref.value, "cooler".to_string());
        } else {
            panic!("Node at index 1 should exist");
        }
    }
}
