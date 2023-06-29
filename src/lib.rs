use std::hash::Hash;

use crate::random::Random;
use crate::round_robin::RoundRobin;
use crate::weighted_round_robin::WeightedRoundRobin;

mod round_robin;
mod random;
mod weighted_round_robin;


pub trait Balancer<T: Hash> {
    fn add_node(&mut self, node: Node<T>);
    fn next(&mut self) -> Option<&Node<T>>;

    fn next_id(&mut self) -> Option<&T> {
        self.next().map(|n| &n.id)
    }
}

pub struct Node<T: Hash> {
    id: T,
    weight: i32,
}

pub enum BalanceType {
    RR,
    WRR,
    Random,
}

pub fn new<'a, T: Hash + 'a>(balance_type: BalanceType, nodes: Vec<Node<T>>) -> Box<dyn Balancer<T> + 'a> {
    match balance_type {
        BalanceType::RR => {
            Box::new(RoundRobin::new(nodes))
        }
        BalanceType::WRR => {
            Box::new(WeightedRoundRobin::new(nodes))
        }
        BalanceType::Random => {
            Box::new(Random::new(nodes))
        }
    }
}