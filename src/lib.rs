use std::fmt::Debug;
use std::hash::Hash;

mod round_robin;
mod random;
mod weighted_round_robin;


trait Balancer<T: Hash> {
    fn add_node(&mut self, node: Node<T>);
    fn next(&mut self) -> Option<&Node<T>>;

    fn next_id(&mut self) -> Option<&T> {
        self.next().map(|n| &n.id)
    }
}

pub struct Node<T: Hash> {
    id: T,
    down: bool,
    weight: i32,
}
