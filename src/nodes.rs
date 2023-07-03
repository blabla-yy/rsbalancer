use std::collections::hash_map::IterMut;
use std::collections::HashMap;
use std::hash::Hash;

use crate::errors::{DuplicatedKeyError, NotFoundError};
use crate::Node;

pub struct NodesContainer<T: Hash + Eq + Copy> {
    vec: Vec<T>,
    map: HashMap<T, Node<T>>,
}

#[allow(dead_code)]
impl<T: Hash + Eq + Copy> NodesContainer<T> {
    pub fn new() -> NodesContainer<T> {
        NodesContainer {
            vec: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn from(nodes: Vec<Node<T>>) -> NodesContainer<T> {
        let mut map = HashMap::new();
        let ids = nodes.into_iter()
            .map(|item| {
                let id = item.id;
                map.insert(item.id, item);
                return id;
            })
            .collect();
        NodesContainer {
            vec: ids,
            map: map,
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn get_all(&self) -> Vec<&Node<T>> {
        self.vec
            .iter()
            .map(|id| self.map.get(id))
            .flatten()
            .collect()
    }

    pub fn insert(&mut self, node: Node<T>) -> Result<(), DuplicatedKeyError> {
        if self.map.contains_key(&node.id) {
            return Err(DuplicatedKeyError);
        }
        self.vec.push(node.id);
        self.map.insert(node.id, node);
        Ok(())
    }

    /// O(n)
    pub fn remove(&mut self, id: &T) -> Result<usize, NotFoundError> {
        match self.map.remove(id) {
            None => {
                Err(NotFoundError)
            }
            Some(_) => {
                if let Some(index) = self.vec.iter().position(|x| x == id) {
                    self.vec.remove(index);
                    Ok(index)
                } else {
                    Err(NotFoundError)
                }
            }
        }
    }

    /// O(1)
    pub fn get_mut_by_id(&mut self, id: &T) -> Option<&mut Node<T>> {
        self.map.get_mut(&id)
    }

    pub fn get_by_id(&self, id: &T) -> Option<&Node<T>> {
        self.map.get(&id)
    }

    /// O(1)
    pub fn get_mut_by_index(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.vec.get(index).map(|id| {
            self.map.get_mut(id)
        }).flatten()
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Node<T>> {
        self.vec.get(index).map(|id| {
            self.map.get(id)
        }).flatten()
    }
    /// O(1)
    pub fn set_down(&mut self, id: &T, down: bool) -> Result<(), NotFoundError> {
        self.get_mut_by_id(id)
            .map(|node| {
                node.down = down
            })
            .ok_or(NotFoundError)
    }


    pub fn iter_mut(&mut self) -> IterMut<'_, T, Node<T>> {
        self.map.iter_mut()
    }
}

// impl<T: Hash + Eq + Copy> NodesContainer<T> {
//     pub fn iter(&mut self) -> NodesContainerIter<T> {
//         NodesContainerIter {
//             container: self,
//             index: 0,
//         }
//     }
// }
//
// pub struct NodesContainerIter<'a, T: Hash + Eq + Copy> {
//     container: &'a mut NodesContainer<T>,
//     index: usize,
// }
//
// impl<'a, T: Hash + Eq + Copy> Iterator for NodesContainerIter<'a, T> {
//     type Item = &'a mut Node<T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index < self.container.vec.len() {
//             let key = self.container.vec[self.index];
//             self.index += 1;
//             self.container.map.get_mut(&key)
//         } else {
//             None
//         }
//     }
// }


#[cfg(test)]
mod nodes_test {
    use crate::Node;
    use crate::nodes::NodesContainer;

    #[test]
    fn simple() {
        let mut nodes: NodesContainer<i32> = NodesContainer::new();

        assert!(nodes.insert(Node::new_with_default_weight(1)).is_ok());
        assert!(nodes.insert(Node::new_with_default_weight(2)).is_ok());
        assert!(nodes.insert(Node::new_with_default_weight(3)).is_ok());
        assert!(nodes.insert(Node::new_with_default_weight(4)).is_ok());
        assert!(nodes.insert(Node::new_with_default_weight(1)).is_err());

        assert_eq!(nodes.get_by_index(0).unwrap().id, 1);
        assert_eq!(nodes.get_by_index(1).unwrap().id, 2);
        assert_eq!(nodes.get_by_index(2).unwrap().id, 3);
        assert_eq!(nodes.get_by_index(3).unwrap().id, 4);

        assert!(nodes.remove(&1).is_ok());
        assert!(nodes.remove(&1).is_err());
        assert!(nodes.get_by_id(&1).is_none());

        assert_eq!(nodes.get_by_index(0).unwrap().id, 2);
        assert!(nodes.set_down(&2, true).is_ok());

        assert_eq!(nodes.get_by_index(0).unwrap().down, true);

        // for item in &nodes {
        //     println!("{}", item.id);
        // }
    }
}