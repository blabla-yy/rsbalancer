use std::hash::Hash;
use std::ops::Deref;

use rand::Rng;

use crate::{Balancer, Node};
use crate::round_robin::RoundRobin;

pub struct Random<T: Hash> {
    nodes: Vec<Node<T>>,
}

impl<T: Hash> Random<T> {
    pub fn new(nodes: Vec<Node<T>>) -> Random<T> {
        Random {
            nodes,
        }
    }
}

impl<T: Hash> Balancer<T> for Random<T> {
    fn add_node(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    fn next(&mut self) -> Option<&Node<T>> {
        let len = self.nodes.len();
        if len == 0 {
            return None;
        }

        let index = rand::thread_rng().gen_range(0..len);
        self.nodes.get(index)
    }
}


#[cfg(test)]
mod random_test {
    use crate::{Balancer, Node};
    use crate::random::Random;

    #[test]
    fn simple() {
        let nodes = vec![1, 2, 3, 4, 5];
        let nodes = nodes.into_iter().map(|id| Node {
            id,
            weight: 1,
            down: false,
        }).collect();
        let mut balancer = Random::new(nodes);
        for _ in 0..50 {
            assert!(balancer.next().is_some());
        }
    }
}
