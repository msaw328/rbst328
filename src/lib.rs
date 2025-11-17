use std::{cmp::{Ordering, PartialOrd}, fmt::Display};

struct Node<V> {
    left: Option<Box<Node<V>>>,
    right: Option<Box<Node<V>>>,
    value: V
}

impl <V: PartialOrd + Copy> Node<V> {
    pub fn new(value: V) -> Self {
        Self {
            left: None,
            right: None,
            value
        }
    }
}

pub struct BST<V: PartialOrd> {
    head: Option<Box<Node<V>>>
}

impl<V: PartialOrd + Copy> BST<V> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    // Returns the position for given value in the tree:
    // If result is None, it means it's a reference to empty leaf node that should be occupied by the value
    // If result is Some, it contains Box'ed Node with given value
    fn find_position(&mut self, value: V) -> &mut Option<Box<Node<V>>> {
        let mut current_node = &mut self.head;

        // Way too convoluted and much too verbose due to: https://stackoverflow.com/a/73740329
        while let Some(node) = current_node.as_mut(){
            current_node = match node.value.partial_cmp(&value).expect("Non-comparable values not allowed") {
                Ordering::Less => &mut current_node.as_mut().unwrap().right,
                Ordering::Greater => &mut current_node.as_mut().unwrap().left,
                Ordering::Equal => break
            };
        }

        current_node
    }
    
    pub fn insert(&mut self, value: V) -> () {
        let found_node = self.find_position(value);

        // This deals with duplicate values - inserting duplicate vlaue is no-op
        if found_node.is_some() {
            return;
        }

        // Found reference to a None option which is a leaf node that should contain the value
        *found_node = Some(Box::new(Node::new(value)));
    }

    

}

impl<T: Display + PartialOrd> BST<T> {
    // TODO: Probably remove this/change to debug-only
    // TODO: clean this code up and change it into a BFS function?
    pub fn pretty_print (&self) -> () {
        let mut current_vector = vec![&self.head];

        let mut space_count = 35;

        while current_vector.iter().any(|node| node.is_some()) {
            print!("{}", std::iter::repeat(" ").take(space_count).collect::<String>());
            for node in &current_vector {
                if node.is_some() {
                    print!("{:>3}   ", node.as_ref().unwrap().value);
                } else {
                    print!("X   ");
                }
            }
            println!("");
        
            let mut next_vector = vec![];
            for node in current_vector {
                if node.is_some() {
                    next_vector.push(&(node.as_ref().unwrap().left));
                    next_vector.push(&(node.as_ref().unwrap().right));
                } else {
                    next_vector.push(&None);
                    next_vector.push(&None);
                }
            }
            current_vector = next_vector;

            space_count -= 3;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_inserts() {
        let mut bst = BST::<i32>::new();
        bst.insert(10);
        bst.insert(25);
        bst.insert(4);
        bst.insert(3);
        bst.insert(15);
        bst.insert(15);
        bst.insert(30);
        bst.insert(2);
        bst.insert(16);

        println!("PRINTING TREE");
        bst.pretty_print();
    }
}
