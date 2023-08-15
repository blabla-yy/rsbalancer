use crate::errors::{DuplicatedKeyError, NotFoundError};
use crate::Node;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

pub struct ConsistentHashing {
    nodes: BTreeMap<u64, String>,
    user_nodes: HashMap<String, Node<String>>,
    replicas: usize,
}

impl ConsistentHashing {
    pub fn new(nodes: Vec<Node<String>>, replicas: usize) -> ConsistentHashing {
        let mut balancer = ConsistentHashing {
            nodes: BTreeMap::new(),
            user_nodes: HashMap::new(),
            replicas,
        };

        for node in nodes {
            // ignore same node
            let _ = balancer.add_node(node);
        }
        balancer
    }
}

impl ConsistentHashing {
    fn replicas_of_node(&self, node: &Node<String>) -> usize {
        let count = node.weight * self.replicas;
        if count <= 0 {
            1
        } else {
            count
        }
    }
    pub fn get_matching_node_id(&self, request: String) -> Option<&String> {
        self.get_matching_node(request).map(|item| &item.id)
    }
    /// return none if empty nodes.
    pub fn get_matching_node(&self, request: String) -> Option<&Node<String>> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        request.hash(&mut hasher);
        let key = hasher.finish();

        match self
            .nodes
            .range(key..)
            .next()
            .or_else(|| self.nodes.iter().next())
        {
            Some((_, id)) => {
                return self.user_nodes.get(id);
            }
            None => {
                return self
                    .nodes
                    .first_key_value()
                    .map(|(_, id)| self.user_nodes.get(id))
                    .flatten();
            }
        }
    }

    fn hash(&self, id: &String) -> u64 {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        hasher.finish()
    }
}

impl ConsistentHashing {
    pub fn add_node(&mut self, node: Node<String>) -> Result<(), DuplicatedKeyError> {
        if self.contains_id(&node.id) {
            return Err(DuplicatedKeyError);
        }
        let count = self.replicas_of_node(&node);
        let id = node.id.clone();
        {
            let key = self.hash(&node.id);
            self.nodes.insert(key, id.clone());

            self.user_nodes.insert(id.clone(), node);
        }
        for i in 1..count {
            let key = self.hash(&format!("{}-{}", &id, i));
            self.nodes.insert(key, id.clone());
        }
        Ok(())
    }

    pub fn remove_node(&mut self, id: &String) -> Result<(), NotFoundError> {
        match self.get_node(id) {
            Some(node) => {
                let count = self.replicas_of_node(&node);
                for i in 0..count {
                    let key = if i == 0 {
                        self.hash(&id)
                    } else {
                        self.hash(&format!("{}-{}", id.to_string(), i))
                    };
                    self.nodes.remove(&key);
                }
                self.user_nodes.remove(id);
                return Ok(());
            }
            None => {
                return Err(NotFoundError);
            }
        };
    }

    pub fn contains_id(&mut self, id: &String) -> bool {
        self.get_node(id).is_some()
    }

    /// miss, return None.
    pub fn get_node(&self, id: &String) -> Option<&Node<String>> {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        let key = hasher.finish();
        self.nodes
            .get(&key)
            .map(|id| self.user_nodes.get(id))
            .flatten()
    }

    pub fn get_nodes(&self) -> Vec<&Node<String>> {
        self.user_nodes.values().collect()
    }
}

#[cfg(test)]
mod consistent_hash_test {
    use crate::Node;

    use super::ConsistentHashing;

    #[test]
    fn get() {
        let balancer = ConsistentHashing::new(
            vec![
                Node::new_with_default_weight("1".to_string()),
                Node::new_with_default_weight("2".to_string()),
                Node::new_with_default_weight("3".to_string()),
            ],
            10,
        );
        assert_eq!(balancer.get_nodes().len(), 3);
        assert!(balancer.get_node(&"1".to_string()).is_some());
        assert!(balancer.get_node(&"4".to_string()).is_none());
    }

    #[test]
    fn remove() {
        let mut balancer = ConsistentHashing::new(
            vec![
                Node::new_with_default_weight("1".to_string()),
                Node::new_with_default_weight("2".to_string()),
                Node::new_with_default_weight("3".to_string()),
            ],
            10,
        );
        balancer.remove_node(&"1".to_string()).unwrap();
        assert!(balancer.get_node(&"1".to_string()).is_none());
        assert!(balancer.get_node(&"2".to_string()).is_some());
    }

    #[test]
    fn simple() {
        let mut balancer = ConsistentHashing::new(
            vec![
                Node::new_with_default_weight("1".to_string()),
                Node::new_with_default_weight("2".to_string()),
                Node::new_with_default_weight("3".to_string()),
            ],
            10,
        );
        let ip = vec!["123", "234", "122"];
        let mut nodes = Vec::with_capacity(3);
        for item in &ip {
            let result = balancer.get_matching_node(item.to_string()).unwrap();
            println!("ip result: {}", result.id);
            for _ in 0..10 {
                assert_eq!(
                    result.id,
                    balancer.get_matching_node(item.to_string()).unwrap().id
                );
            }
            nodes.push(result.id.clone());
        }

        balancer.remove_node(&nodes.first().unwrap()).unwrap();
        assert_eq!(2, balancer.get_nodes().len());

        let balancer = balancer;
        let first_ip = ip.first().unwrap();
        let node = balancer.get_matching_node(first_ip.to_string()).unwrap();
        assert_ne!(node.id, nodes.first().unwrap().clone());
    }
}
