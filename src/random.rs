use std::hash::Hash;

use rand::Rng;

use crate::{Balancer, Node};
use crate::errors::{DuplicatedKeyError, NotFoundError};
use crate::nodes::NodesContainer;

pub struct Random<T: Hash + Eq + Clone> {
    nodes: NodesContainer<T>,
}

impl<T: Hash + Eq + Clone> Random<T> {
    pub fn new(nodes: Vec<Node<T>>) -> Random<T> {
        Random {
            nodes: NodesContainer::from(nodes),
        }
    }
}

impl<T: Hash + Eq + Clone> Balancer<T> for Random<T> {
    fn add_node(&mut self, node: Node<T>) -> Result<(), DuplicatedKeyError> {
        self.nodes.insert(node)
    }

    fn remove_node(&mut self, id: &T) -> Result<(), NotFoundError> {
        self.nodes.remove(id).map(|_| ())
    }

    fn contains_id(&mut self, id: &T) -> bool {
        self.nodes.get_by_id(id).is_some()
    }

    fn get_node(&self, id: &T) -> Option<&Node<T>> {
        self.nodes.get_by_id(id)
    }

    fn get_nodes(&self) -> Vec<&Node<T>> {
        self.nodes.get_all()
    }

    fn set_down(&mut self, id: &T, down: bool) -> Result<(), NotFoundError> {
        self.nodes.set_down(id, down)
    }

    fn next_id(&mut self) -> Option<&T> {
        self.next().map(|n| &n.id)
    }

    fn next(&mut self) -> Option<&Node<T>> {
        let len = self.nodes.len();
        if len == 0 {
            return None;
        }

        let mut index = rand::thread_rng().gen_range(0..len);
        let init = index;
        //todo
        while let Some(node) = self.nodes.get_by_index(index) {
            if node.is_down() {
                index = if index >= len - 1 {
                    0
                } else {
                    index + 1
                };
                // all is down.
                if index == init {
                    break;
                }
                continue;
            }
            return Some(node);
        }
        return None;
    }
}


#[cfg(test)]
mod random_test {
    use crate::{Balancer, Node};
    use crate::random::Random;

    #[test]
    fn simple() {
        let nodes = vec![1, 2, 3, 4, 5];
        let nodes = nodes.into_iter().map(|id| Node::new_with_default_weight(id)).collect();
        let mut balancer = Random::new(nodes);
        for _ in 0..50 {
            assert!(balancer.next().is_some());
        }
    }

    #[test]
    fn down() {
        let nodes = vec![1, 2];
        let nodes = nodes.into_iter().map(|id| Node::new_with_default_weight(id)).collect();
        let mut balancer = Random::new(nodes);
        assert!(balancer.next().is_some());

        balancer.set_down(&1, true).unwrap();
        assert_eq!(*balancer.next_id().unwrap(), 2);
        assert_eq!(*balancer.next_id().unwrap(), 2);

        balancer.set_down(&2, true).unwrap();

        assert!(balancer.next_id().is_none());
        assert!(balancer.next_id().is_none());
    }
}
