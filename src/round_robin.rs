use std::hash::Hash;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{Balancer, Node};

pub struct RoundRobin<T: Hash> {
    nodes: Vec<Node<T>>,
    index: Arc<AtomicUsize>,
}

impl<T: Hash> RoundRobin<T> {
    pub fn new(nodes: Vec<Node<T>>) -> RoundRobin<T> {
        RoundRobin {
            nodes,
            index: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl<T: Hash> Balancer<T> for RoundRobin<T> {
    fn add_node(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    fn next(&mut self) -> Option<&Node<T>> {
        let len = self.nodes.len();
        if len == 0 {
            return None;
        }
        let result = self.index.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |val| {
            if val >= len - 1 {
                Some(0)
            } else {
                Some(val + 1)
            }
        });
        let i = result.unwrap();
        self.nodes.get(i)
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
                balancer.add_node(Node::new_with_default_weight(4));
            }
            println!("{}", id);
            assert_eq!((i % 4) + 1, id);
        }
    }

    // fn increase(map: &mut HashMap<i32, i32>, key: i32, value: i32) {
    //     match map.get(&key) {
    //         None => {
    //             map.insert(key, value);
    //         }
    //         Some(count) => {
    //             map.insert(key, count + value);
    //         }
    //     }
    // }
    //
    // #[test]
    // fn multithreading() {
    //     let threads = 10;
    //     let count = 20;
    //
    //
    //     let nodes = vec![1, 2, 3, 4, 5];
    //     let balancer = Arc::new(RoundRobin::new(nodes));
    //     let mut handlers = Vec::new();
    //
    //     let map = Arc::new(Mutex::new(HashMap::new()));
    //     for _ in 0..threads {
    //         let balancer = balancer.clone();
    //
    //         let map = map.clone();
    //         handlers.push(std::thread::spawn(move || {
    //             let mut thread_map = HashMap::new();
    //             for _ in 0..count {
    //                 let result = balancer.next().unwrap();
    //                 increase(&mut thread_map, *result, 1);
    //             }
    //             let mut m = map.lock().unwrap();
    //             for (k, v) in thread_map {
    //                 increase(&mut m, k, v);
    //             }
    //         }));
    //     }
    //
    //     for handler in handlers {
    //         handler.join().unwrap();
    //     }
    //     let m = map.lock().unwrap();
    //     let expected = threads * count / 5;
    //     for (_, v) in &*m {
    //         assert_eq!(*v, expected);
    //     }
    // }
}
