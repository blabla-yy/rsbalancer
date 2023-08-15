use std::hash::Hash;

use crate::errors::{DuplicatedKeyError, NotFoundError};
use crate::random::Random;
use crate::round_robin::RoundRobin;
use crate::weighted_round_robin::WeightedRoundRobin;

mod round_robin;
mod random;
mod weighted_round_robin;
mod nodes;
mod errors;
mod consistent_hash;


pub trait Balancer<T: Hash + Eq + Clone> {
    fn add_node(&mut self, node: Node<T>) -> Result<(), DuplicatedKeyError>;
    fn remove_node(&mut self, id: &T) -> Result<(), NotFoundError>;
    fn contains_id(&mut self, id: &T) -> bool;
    fn get_node(&self, id: &T) -> Option<&Node<T>>;
    fn get_nodes(&self) -> Vec<&Node<T>>;

    fn set_down(&mut self, id: &T, down: bool) -> Result<(), NotFoundError>;
    fn next(&mut self) -> Option<&Node<T>>;
    fn next_id(&mut self) -> Option<&T> {
        self.next().map(|n| &n.id)
    }
}

pub struct Node<T: Hash + Eq + Clone> {
    id: T,
    weight: usize,
    down: bool,
    current_weight: i32,
    effective_weight: i32,
}

impl<T: Hash + Eq + Clone> Node<T> {
    pub fn new_with_default_weight(id: T) -> Node<T> {
        Node {
            id,
            weight: 1,
            down: false,
            current_weight: 0,
            effective_weight: 1,
        }
    }

    pub fn new(id: T, weight: usize) -> Node<T> {
        Node {
            id,
            weight,
            down: false,
            current_weight: 0,
            effective_weight: weight as i32,
        }
    }

    pub fn get_id(&self) -> T {
        self.id.clone()
    }

    pub fn get_weight(&self) -> usize {
        self.weight
    }

    pub fn is_down(&self) -> bool {
        self.down
    }
}

pub enum BalancerEnum {
    /// Round-Robin
    /// O(1)
    RR,
    /// Smooth Weighted Round-Robin
    /// O(n)
    WRR,
    /// Random
    /// O(1)
    Random,
    /// ConsistentHash
    ConsistentHash
}

pub fn new<'a, T: Hash + Eq + Clone + 'a>(balancer_enum: BalancerEnum, nodes: Vec<Node<T>>) -> Box<dyn Balancer<T> + 'a> {
    match balancer_enum {
        BalancerEnum::RR => {
            Box::new(RoundRobin::new(nodes))
        }
        BalancerEnum::WRR => {
            Box::new(WeightedRoundRobin::new(nodes))
        }
        BalancerEnum::Random => {
            Box::new(Random::new(nodes))
        }
        BalancerEnum::ConsistentHash => todo!(),
    }
}

