use std::hash::Hash;

use crate::{Balancer, Node};

pub struct WeightedRoundRobin<T: Hash> {
    nodes: Vec<WeightedNode<T>>,
}

impl<T: Hash> WeightedRoundRobin<T> {
    pub fn new(nodes: Vec<Node<T>>) -> WeightedRoundRobin<T> {
        let nodes = nodes.into_iter()
            .map(|node| {
                WeightedNode::new(node)
            })
            .collect();
        WeightedRoundRobin {
            nodes
        }
    }
}

struct WeightedNode<T: Hash> {
    data: Node<T>,
    current_weight: i32,
    effective_weight: i32,
}

impl<T: Hash> WeightedNode<T> {
    fn new(data: Node<T>) -> WeightedNode<T> {
        let weight = data.weight;
        WeightedNode {
            data,
            current_weight: 0,
            effective_weight: weight,
        }
    }
}

impl<T: Hash> Balancer<T> for WeightedRoundRobin<T> {
    fn add_node(&mut self, node: Node<T>) {
        self.nodes.push(WeightedNode::new(node));
    }

    fn next(&mut self) -> Option<&Node<T>> {
        let len = self.nodes.len();
        if len == 0 {
            return None;
        }
        let mut total = 0;
        let mut result = None;
        for mut node in &mut self.nodes {
            node.current_weight += node.effective_weight;
            total += node.effective_weight;
            if node.effective_weight < node.data.weight {
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
            return &node.data;
        });
    }
}

#[cfg(test)]
mod weighted_round_robin_test {
    use std::collections::HashMap;

    use crate::{Balancer, Node};
    use crate::weighted_round_robin::WeightedRoundRobin;

    fn map_nodes(array: Vec<(i32, i32)>) -> Vec<Node<i32>> {
        array.into_iter()
            .map(|(id, weight)| {
                Node {
                    id: id,
                    down: false,
                    weight,
                }
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
        for i in 0..10 {
            let id = balancer.next().unwrap().id;
            println!("id:{}", id);
            map.entry(id).and_modify(|v| *v += 1);
            if i == 1 {
                balancer.add_node(Node {
                    id: 4,
                    down: false,
                    weight: 1,
                });
            }
        }
        for (i, v) in map {
            if i == 1 {
                assert_eq!(5, v);
            }
            if i == 2 {
                assert_eq!(3, v);
            }
            if i == 3 {
                assert_eq!(1, v);
            }
            if i == 4 {
                assert_eq!(1, v);
            }
        }
    }
}