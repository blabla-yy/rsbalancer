use std::hash::Hash;

use crate::{Balancer, Node};
use crate::errors::{DuplicatedKeyError, NotFoundError};
use crate::nodes::NodesContainer;

pub struct RoundRobin<T: Hash + Eq + Clone> {
    nodes: NodesContainer<T>,
    index: usize,
}

impl<T: Hash + Eq + Clone> RoundRobin<T> {
    pub fn new(nodes: Vec<Node<T>>) -> RoundRobin<T> {
        RoundRobin {
            nodes: NodesContainer::from(nodes),
            index: 0,
        }
    }
}

impl<T: Hash + Eq + Clone> Balancer<T> for RoundRobin<T> {
    fn add_node(&mut self, node: Node<T>) -> Result<(), DuplicatedKeyError> {
        self.nodes.insert(node)
    }

    fn remove_node(&mut self, id: &T) -> Result<(), NotFoundError> {
        self.nodes.remove(id)
            .map(|index| {
                if self.index > index {
                    self.index -= 1;
                } else if self.index == index {
                    self.index = if self.index >= self.nodes.len() - 1 {
                        0
                    } else {
                        self.index + 1
                    };
                }
            })
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

    fn next(&mut self) -> Option<&Node<T>> {
        let len = self.nodes.len();
        if len == 0 {
            return None;
        }
        let init = self.index;
        //todo
        while let Some(node) = self.nodes.get_by_index(self.index) {
            self.index = if self.index >= len - 1 {
                0
            } else {
                self.index + 1
            };
            if node.is_down() {
                // all is down.
                if self.index == init {
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
mod round_robin_test {
    use crate::{Balancer, Node};
    use crate::round_robin::RoundRobin;

    #[test]
    fn simple() {
        let nodes = vec![1, 2, 3, 4, 5];
        let nodes = nodes.into_iter().map(|id| Node::new_with_default_weight(id)).collect();
        let mut balancer = RoundRobin::new(nodes);
        for i in 0..20 {
            assert_eq!((i % 5) + 1, balancer.next().unwrap().id);
        }
    }

    #[test]
    fn add_node() {
        let nodes = vec![1, 2, 3];
        let nodes = nodes.into_iter().map(|id| Node::new_with_default_weight(id)).collect();
        let mut balancer = RoundRobin::new(nodes);
        for i in 0..10 {
            let id = balancer.next().unwrap().id;
            if i == 1 {
                balancer.add_node(Node::new_with_default_weight(4)).unwrap();
            }
            assert_eq!((i % 4) + 1, id);
        }
    }

    #[test]
    fn remove_node() {
        let nodes = vec![1, 2, 3];
        let nodes = nodes.into_iter().map(|id| Node::new_with_default_weight(id)).collect();
        let mut balancer = RoundRobin::new(nodes);

        assert_eq!(*balancer.next_id().unwrap(), 1);
        balancer.remove_node(&1).unwrap();
        assert_eq!(*balancer.next_id().unwrap(), 2);
        balancer.remove_node(&3).unwrap();
        assert_eq!(*balancer.next_id().unwrap(), 2);
    }

    #[test]
    fn down() {
        let nodes = vec![1, 2, 3];
        let nodes = nodes.into_iter().map(|id| Node::new_with_default_weight(id)).collect();
        let mut balancer = RoundRobin::new(nodes);

        balancer.set_down(&1, true).unwrap();
        assert_eq!(*balancer.next_id().unwrap(), 2);
        assert_eq!(*balancer.next_id().unwrap(), 3);
        assert_eq!(*balancer.next_id().unwrap(), 2);

        balancer.set_down(&2, true).unwrap();
        balancer.set_down(&3, true).unwrap();

        assert!(balancer.next_id().is_none());
        assert!(balancer.next_id().is_none());
    }
}
