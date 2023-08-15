use crate::errors::{DuplicatedKeyError, NotFoundError};
use crate::{Balancer, Node};
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{self, Hash, Hasher};

pub struct ConsistentHash<T: Hash + Eq + Copy + ToString> {
    nodes: BTreeMap<u64, Node<T>>,
    replicas: usize,
}

impl<T: Hash + Eq + Copy + ToString> ConsistentHash<T> {
    pub fn new(
        nodes: Vec<Node<T>>,
        replicas: usize,
    ) -> Result<ConsistentHash<T>, DuplicatedKeyError> {
        let mut balancer = ConsistentHash {
            nodes: BTreeMap::new(),
            replicas,
        };

        for node in nodes {
            balancer.add_node(node)?;
        }
        Ok(balancer)
    }
}

impl<T: Hash + Eq + Copy + ToString> ConsistentHash<T> {
    fn replicas_of_node(&self, node: &Node<T>) -> usize {
        let count = node.weight * self.replicas;
        if count <= 0 {
            1
        } else {
            count
        }
    }

    fn get(&self, id: String) -> Option<&Node<T>> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        let key = hasher.finish();

        match self
            .nodes
            .range(key..)
            .next()
            .or_else(|| self.nodes.iter().next())
        {
            Some(node) => {
                return Some(node.1);
            }
            None => {
                return self.nodes.first_key_value().map(|item| item.1);
            }
        }
    }

    fn hash(&self, id: String) -> u64 {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        hasher.finish()
    }

    fn hash_id(&self, id: T) -> u64 {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: Hash + Eq + Copy + ToString> Balancer<T> for ConsistentHash<T> {
    fn add_node(&mut self, node: Node<T>) -> Result<(), DuplicatedKeyError> {
        if self.contains_id(&node.id) {
            return Err(DuplicatedKeyError);
        }
        let count = self.replicas_of_node(&node);
        let id = node.id;
        for i in 0..count {
            let key = if i == 0 {
                self.hash_id(id)
            } else {
                self.hash(format!("{}-{}", id.to_string(), i))
            };
            self.nodes.insert(key, Node::new_with_default_weight(id));
        }
        Ok(())
    }

    fn remove_node(&mut self, id: &T) -> Result<(), NotFoundError> {
        match self.get_node(id) {
            Some(node) => {
                let count = self.replicas_of_node(&node);
                for i in 0..count {
                    let mut hasher = DefaultHasher::new();
                    if i == 0 {
                        id.hash(&mut hasher);
                    } else {
                        format!("{}-{}", id.to_string(), i).hash(&mut hasher);
                    };
                    let key = hasher.finish();
                    self.nodes.remove(&key);
                }
                return Ok(());
            }
            None => {
                return Err(NotFoundError);
            }
        };
    }

    fn contains_id(&mut self, id: &T) -> bool {
        self.get_node(id).is_some()
    }

    fn get_node(&self, id: &T) -> Option<&Node<T>> {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        let key = hasher.finish();
        self.nodes.get(&key)
    }

    fn get_nodes(&self) -> Vec<&Node<T>> {
        todo!()
    }

    fn set_down(&mut self, id: &T, down: bool) -> Result<(), NotFoundError> {
        todo!()
    }

    fn next(&mut self) -> Option<&Node<T>> {
        todo!()
    }
}

#[cfg(test)]
mod consistent_hash_test {
    use crate::Node;

    use super::ConsistentHash;

    #[test]
    fn simple() {
        let mut balancer = ConsistentHash::new(
            vec![
                Node::new_with_default_weight("1"),
                Node::new_with_default_weight("2"),
                Node::new_with_default_weight("3"),
            ],
            10,
        )
        .unwrap();

        let ip = vec!["123", "234", "122"];
        for item in ip {
            let result = balancer.get(item.to_string()).unwrap();
            println!("ip result: {}", result.id);
            for _ in 0..10 {
                assert_eq!(result.id, balancer.get(item.to_string()).unwrap().id);
            }
        }
    }

    #[test]
    fn add_node() {}

    #[test]
    fn down() {}
}
