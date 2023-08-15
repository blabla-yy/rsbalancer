use std::hash::Hash;

use consistent_hash::ConsistentHashing;

use crate::errors::{DuplicatedKeyError, NotFoundError};
use crate::random::Random;
use crate::round_robin::RoundRobin;
use crate::weighted_round_robin::WeightedRoundRobin;

mod consistent_hash;
mod errors;
mod nodes;
mod random;
mod round_robin;
mod weighted_round_robin;

pub trait Balancer<T: Hash + Eq + Clone> {
    fn add_node(&mut self, node: Node<T>) -> Result<(), DuplicatedKeyError>;
    fn remove_node(&mut self, id: &T) -> Result<(), NotFoundError>;
    fn contains_id(&mut self, id: &T) -> bool;
    fn get_node(&self, id: &T) -> Option<&Node<T>>;
    fn get_nodes(&self) -> Vec<&Node<T>>;

    fn set_down(&mut self, id: &T, down: bool) -> Result<(), NotFoundError>;

    fn next(&mut self) -> Option<&Node<T>>;
    fn next_id(&mut self) -> Option<&T>;
}

pub struct Node<T: Hash + Eq + Clone> {
    id: T,
    weight: usize,
    down: bool,
    current_weight: i32,
    effective_weight: i32,
}

impl<T: Hash + Eq + Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            weight: self.weight.clone(),
            down: self.down.clone(),
            current_weight: self.current_weight.clone(),
            effective_weight: self.effective_weight.clone(),
        }
    }
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

    pub fn get_id(&self) -> &T {
        &self.id
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
    RR,
    /// Smooth Weighted Round-Robin
    WRR,
    /// Random
    Random,
}

pub fn new<'a, T: Hash + Eq + Clone + 'a>(
    balancer_enum: BalancerEnum,
    nodes: Vec<Node<T>>,
) -> Box<dyn Balancer<T> + 'a> {
    match balancer_enum {
        BalancerEnum::RR => Box::new(RoundRobin::new(nodes)),
        BalancerEnum::WRR => Box::new(WeightedRoundRobin::new(nodes)),
        BalancerEnum::Random => Box::new(Random::new(nodes)),
    }
}

pub fn weighted_round_robin<T: Hash + Eq + Clone>(nodes: Vec<Node<T>>) -> WeightedRoundRobin<T> {
    WeightedRoundRobin::new(nodes)
}

pub fn round_robin<T: Hash + Eq + Clone>(nodes: Vec<Node<T>>) -> RoundRobin<T> {
    RoundRobin::new(nodes)
}

pub fn random<T: Hash + Eq + Clone>(nodes: Vec<Node<T>>) -> Random<T> {
    Random::new(nodes)
}

/// ConsistentHashing
/// number of virtual nodes: replicas * node.weight.
/// node.down does not work in ConsistentHash now.
pub fn consistent_hashing(nodes: Vec<Node<String>>, replicas: usize) -> ConsistentHashing {
    ConsistentHashing::new(nodes, replicas)
}
