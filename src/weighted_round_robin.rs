use std::hash::Hash;

use crate::{Balancer, Node};
use crate::errors::{DuplicatedKeyError, NotFoundError};
use crate::nodes::NodesContainer;

pub struct WeightedRoundRobin<T: Hash + Eq + Clone> {
    nodes: NodesContainer<T>,
}

impl<T: Hash + Eq + Clone> WeightedRoundRobin<T> {
    pub fn new(nodes: Vec<Node<T>>) -> WeightedRoundRobin<T> {
        WeightedRoundRobin {
            nodes: NodesContainer::from(nodes),
        }
    }
}

impl<T: Hash + Eq + Clone> Balancer<T> for WeightedRoundRobin<T> {
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

    fn next(&mut self) -> Option<&Node<T>> {
        let len = self.nodes.len();
        if len == 0 {
            return None;
        }
        let mut total = 0;
        let mut result = None;
        for (_, node) in self.nodes.iter_mut() {
            if node.is_down() {
                continue;
            }
            node.current_weight += node.effective_weight;
            total += node.effective_weight;
            if node.effective_weight < (node.weight as i32) {
                node.effective_weight += 1;
            }
            if result.is_none() {
                result = Some(node);
            } else {
                result = result.map(|existed| {
                    if existed.current_weight < node.current_weight {
                        node
                    } else {
                        existed
                    }
                });
            }
        }

        return result.map(|node| {
            node.current_weight -= total;
            return &*node;
        });
    }
}

#[cfg(test)]
mod weighted_round_robin_test {
    use std::collections::HashMap;

    use crate::{Balancer, Node};
    use crate::weighted_round_robin::WeightedRoundRobin;

    fn map_nodes(array: Vec<(i32, usize)>) -> Vec<Node<i32>> {
        array.into_iter()
            .map(|(id, weight)| {
                Node::new(id, weight)
            })
            .collect()
    }

    #[test]
    fn simple() {
        let mut balancer = WeightedRoundRobin::new(map_nodes(vec![
            (1, 3),
            (2, 2),
            (3, 1),
        ]));

        let mut map = HashMap::from([(1, 0), (2, 0), (3, 0)]);
        for _ in 0..12 {
            map.entry(balancer.next().unwrap().id).and_modify(|v| *v += 1);
        }
        for (i, v) in map {
            if i == 1 {
                assert_eq!(6, v);
            }
            if i == 2 {
                assert_eq!(4, v);
            }
            if i == 3 {
                assert_eq!(2, v);
            }
        }
    }

    #[test]
    fn add_node() {
        let mut balancer = WeightedRoundRobin::new(map_nodes(vec![
            (1, 3),
            (2, 2),
            (3, 1),
        ]));

        let mut map = HashMap::from([(1, 0), (2, 0), (3, 0)]);
        let mut prev = -1;
        for i in 0..14 {
            let id = balancer.next().unwrap().id;
            assert_ne!(id, prev);
            prev = id;
            map.entry(id).and_modify(|v| *v += 1);
            if i == 1 {
                balancer.add_node(Node::new(4, 1)).unwrap();
            }
        }
        for (i, v) in map {
            if i == 1 {
                assert_eq!(6, v);
            }
            if i == 2 {
                assert_eq!(4, v);
            }
            if i == 3 {
                assert_eq!(2, v);
            }
            if i == 4 {
                assert_eq!(2, v);
            }
        }
    }


    #[test]
    fn down() {
        let nodes = vec![1, 2, 3];
        let nodes = nodes.into_iter().map(|id| Node::new_with_default_weight(id)).collect();
        let mut balancer = WeightedRoundRobin::new(nodes);

        balancer.set_down(&1, true).unwrap();
        assert_ne!(*balancer.next_id().unwrap(), 1);
        assert_ne!(*balancer.next_id().unwrap(), 1);
        assert_ne!(*balancer.next_id().unwrap(), 1);

        balancer.set_down(&2, true).unwrap();
        balancer.set_down(&3, true).unwrap();

        assert!(balancer.next_id().is_none());
        assert!(balancer.next_id().is_none());
    }
}